use std::fmt::Display;

#[allow(unused)]
use crate::{
    decode::{ Bytes, DecodeError },
    instruction::{
        Instruction,
        encoding::operands::{Operand, Offset, Register},
    }
};


#[derive(Clone, Debug, PartialEq, Default)]
pub struct Line {
    /// Labeled
    labeled: bool, 
    /// Address
    address: Offset, 
    /// Instruction (Trait)
    instruction: Bytes
}
impl Line{
    fn string_with_width(&self, width: usize) -> String {
        let label = if self.labeled { format!("{}:\n", self.address) }
        else { "".into() };

        let bytes = self.instruction.bytes();
        let address = &self.address.0;
        let instruction = self.instruction.to_string();
        let width = width + 4;

        format!("{label}{address:08X}: {bytes: <width$} {instruction}", width = width)
    }
}
impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = if self.labeled { format!("{}h:\n", self.address) }
        else { "".into() };

        let bytes = self.instruction.bytes();
        let instruction = self.instruction.to_string();

        write!(f, "{label}{bytes} {instruction}")
    }
}


#[derive(Clone, Debug, PartialEq, Default)]
pub struct Output {
    lines: Vec<Option<Line>>,
    pointer: Offset,
    width: usize
}

#[allow(unused)]
impl Output {
    pub fn new(bytes: usize) -> Output {
        let mut lines: Vec<Option<Line>> = Vec::with_capacity(bytes);
        for _ in 0..bytes { lines.push(None); }

        Output {lines, ..Default::default() }
    }

    fn update<F>(&mut self, offset: Offset, update: F) -> Result<(), DecodeError>
        where F: FnOnce(&mut Line)
    { 
        let insert_line = self.lines.get_mut(offset.0 as usize);
        if insert_line.is_some() {
            let line = insert_line.unwrap();
            //if line.is_some() {
                //let l = line.as_mut().unwrap();
                //*l = f(l).clone();
            //}
            //let mut line = if line.is_some() { line.as_mut().unwrap().clone() }
            //else { Line::default() };
            //self.lines[offset.0 as usize] = Some(f(&mut line));
            let l = line.get_or_insert_with(|| Line::default());
            update(l);
            Ok(())
        } 
        else { 
            Err(DecodeError::InvalidAddress(offset.0) )
        }
    }

    /// Add instruction to the Output at the location pointed 
    /// by the internal Offset pointer
    pub fn add(&mut self, i: Bytes) -> Result<(), DecodeError> {
        let address = &self.pointer.clone();
        self.update(self.pointer.clone(),
            |line| { 
                line.instruction = i.clone(); 
                line.address = address.clone();
            })?;
                              
        self.pointer.increment(i.length() as u32);
        let width = format!("{}", i.bytes()).len();
        if self.width < width { self.width = width; }
        Ok(())
    }

    pub fn label(&mut self, offset: Offset) -> Result<(), DecodeError> {
        self.update(offset, |line| { 
                line.labeled = true;
            })?;
        Ok(())
    }
}
impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.width;
        let lines = self.lines
            .iter()
            .filter(|l| l.is_some())
            .map(|line| {
                let l = line.clone().expect("Filter should eleminate `None`s");
                let string = format!("{}", l.string_with_width(width));
                string
            })
            .collect::<Vec<String>>()
            .join("\n");

        println!("{lines}");
        write!(f, "{}", lines)
    }
}



#[test]
fn single_line() {
    let expected = vec![
        "00000000: 74 0F     jz offset_00000018h"
    ].join("\n");
    let mut output = Output::new(11);

    let mut instruction = Instruction::new("jz");
    instruction.add(Operand::Label(Offset(0x18)));

    let jz   = Bytes::Decoded {bytes: vec![0x74, 0x0F], instruction: instruction.clone() };

    let _ = output.add(jz);
    assert_eq!(format!("{output}"), expected);
}

#[test]
fn unknown_byte() {
    let expected = vec![
        "00000000: 58     pop eax",
        "00000001: 8F     db 0x8F",
        "00000002: C0     db 0xC0",                
    ].join("\n");

    let mut instruction = Instruction::new("pop");
    instruction.add(Operand::Register(Register::EAX));
    let pop  = Bytes::Decoded { 
        bytes: vec![0x58], 
        instruction: instruction.clone()
    };
    let ub1 = Bytes::Uknown(0x8f);
    let ub2 = Bytes::Uknown(0xC0);
    let mut output = Output::new(3);

    let _ = output.add(pop);
    let _ = output.add(ub1);
    let _ = output.add(ub2);

    assert_eq!(format!("{output}"), expected);
}

#[test]
fn multiple_line() {
    let expected = vec![
        "00000000: 74 0F     jz offset_00000018h",
        "00000002: 01 D1     add ecx, edx",                
        //"00000004: 8B 4D 0C     mov ecx,[ebp+0x0000000c]",
    ].join("\n");

    let mut jz = Instruction::new("jz");
    jz.add(Operand::Label(Offset(0x18)));
    println!("{jz}");
    let jz   = Bytes::Decoded { 
        bytes: vec![0x74, 0x0F], 
        instruction: jz.clone()
    };

    let mut add = Instruction::new("add");
    add.add(Operand::Register(Register::ECX))
       .add(Operand::Register(Register::EDX));
    println!("{add}");
    let add  = Bytes::Decoded { 
        bytes: vec![0x01, 0xD1],
        instruction: add.clone()
    };                 
    // Fix once I implement Displacement
    //let mut mov = Instruction::new("mov");
    //mov.add(Operand::));
    //let mova = Bytes::Decoded { 
        //bytes: vec![0x8B, 0x4D,0x0C],
        //instruction: "mov ecx,[ebp+0x0000000c]".into()
    //};
    

    let mut output = Output::new(11);

    let _ = output.add(jz);
    let _ = output.add(add);
    //let _ = output.add(mova);

    assert_eq!(format!("{output}"), expected);

}

#[test]
fn with_label() {
    let expected = vec![
        "00000000: 74 0F     jz offset_00000018h",
        "offset_00000002h:",
        "00000002: 01 D1     add ecx, edx",                
    ].join("\n");               

    let mut jz = Instruction::new("jz");
    jz.add(Operand::Label(Offset(0x18)));
    let jz   = Bytes::Decoded { 
        bytes: vec![0x74, 0x0F], 
        instruction: jz.clone()
    };

    let mut add = Instruction::new("add");
    add.add(Operand::Register(Register::ECX))
       .add(Operand::Register(Register::EDX));
    let add  = Bytes::Decoded { 
        bytes: vec![0x01, 0xD1],
        instruction: add.clone()
    };                 

    let mut output = Output::new(11);

    let _ = output.add(jz);
    let _ = output.label(Offset(0x2));
    let _ = output.add(add);
    assert_eq!(format!("{output}"), expected);
}
