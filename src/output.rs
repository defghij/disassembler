use std::fmt::Display;

use crate::decode::DecodeError;
use crate::instruction::Instruction;


#[derive(Clone, Debug, PartialEq, Default)]
pub struct Line {
    /// Labeled
    labeled: bool, 
    /// Address
    address: Offset, 
    /// Instruction (Trait)
    instruction: DecodedBytes
}
impl Line{
    pub fn update<F>(&mut self, f: F) -> &mut Self where F: FnOnce(&mut Self) -> &mut Self {
        f(self)
    }
    fn string_with_width(&self, width: usize) -> String {
        let label = if self.labeled { format!("{}h:\n", self.address) }
        else { "".into() };

        let bytes = self.instruction.bytes();
        let address = &self.address.0;
        let instruction = self.instruction.instruction();
        let width = width * 2 + 4;

        format!("{label}{address:08X}: {bytes: <width$} {instruction}", width = width)
    }
}
impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = if self.labeled { format!("{}h:\n", self.address) }
        else { "".into() };

        let bytes = self.instruction.bytes();
        let instruction = self.instruction.instruction();

        write!(f, "{label}{bytes} {instruction}")
    }
}


#[derive(Clone, Debug, PartialEq, Default)]
pub struct Offset(pub u32);
impl Offset {
    pub fn increment(&mut self, bytes: u32) { self.0 += bytes; }
}
impl Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "offset_{:08x}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct OutputLines {
    lines: Vec<Option<Line>>,
    pointer: Offset,
    width: usize
}
impl OutputLines {
    pub fn new(bytes: usize) -> OutputLines {
        let mut lines: Vec<Option<Line>> = Vec::with_capacity(bytes);
        for _ in 0..bytes { lines.push(None); }

        OutputLines {lines, ..Default::default() }
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
    pub fn add(&mut self, i: DecodedBytes) -> Result<(), DecodeError> {
        let address = &self.pointer.clone();
        self.update(self.pointer.clone(),
            |line| { 
                line.instruction = i.clone(); 
                line.address = address.clone();
            })?;
                              
        self.pointer.increment(i.length() as u32);
        if self.width < i.length() { self.width = i.length(); }
        Ok(())
    }

    pub fn label(&mut self, offset: Offset) -> Result<(), DecodeError> {
        self.update(offset, |line| { 
                line.labeled = true;
            })?;
        Ok(())
    }
}
impl Display for OutputLines {
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

        write!(f, "{}", lines)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DecodedBytes {
    bytes: Vec<u8>,
    instruction: String,
}
impl DecodedBytes {
    pub fn new(bytes: Vec<u8>, instruction: String) -> DecodedBytes {
        DecodedBytes { bytes, instruction }
    }
}
impl Instruction for DecodedBytes {
    fn instruction(&self) -> String {
        self.instruction.clone()
    }

    fn bytes(&self) -> String {
        self.bytes
            .iter().map(|b| format!("{b:02X}") )
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn length(&self) -> usize {
        self.bytes.len()
    }

    fn operands(&self) -> Vec<String> {
        unimplemented!("lol");
    }
}

#[test]
fn single_line() {
    let expected = vec![
        "00000000: 74 0F    jz offset_00000018h"
    ].join("\n");
    let mut output = OutputLines::new(11);

    let jz   = DecodedBytes::new(vec![0x74, 0x0F],      "jz offset_00000018h".into());

    let _ = output.add(jz);
    assert_eq!(format!("{output}"), expected);
}

#[test]
fn illegal_instruction() {
}

#[test]
fn unknown_byte() {
}

#[test]
fn multiple_line() {
    let expected = vec![
        "00000000: 74 0F      jz offset_00000018h",
        "00000002: 8B 4D 0C   mov ecx,[ebp+0x0000000c]",
        "00000005: 01 D1      add ecx,edx",                
    ].join("\n");

    let jz   = DecodedBytes::new(vec![0x74, 0x0F],      "jz offset_00000018h".into());
    let mova = DecodedBytes::new(vec![0x8B, 0x4D,0x0C], "mov ecx,[ebp+0x0000000c]".into());    
    let add  = DecodedBytes::new(vec![0x01, 0xD1],      "add ecx,edx".into());                 

    let mut output = OutputLines::new(11);

    let _ = output.add(jz);
    let _ = output.add(mova);
    let _ = output.add(add);

    assert_eq!(format!("{output}"), expected);

}

#[test]
fn with_label() {
    let expected = vec![
        "00000000: 74 0F    jz offset_00000018h",
        "offset_00000002h:",
        "00000002: 01 D1    add ecx,edx",                
    ].join("\n");               

    let jz   = DecodedBytes::new(vec![0x74, 0x0F],      "jz offset_00000018h".into());
    let add  = DecodedBytes::new(vec![0x01, 0xD1],      "add ecx,edx".into());                 

    let mut output = OutputLines::new(11);

    let _ = output.add(jz);
    let _ = output.label(Offset(0x2));
    let _ = output.add(add);
    assert_eq!(format!("{output}"), expected);
}
