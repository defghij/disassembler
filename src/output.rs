use std::fmt::Display;

use tracing::{debug, error, info};

use crate::{instruction::encoding::Sib, opcodes::DecodeRules};
#[allow(unused)]
use crate::{
    decode::{ Bytes, DecodeError },
    instruction::{
        Instruction,
        encoding::operands::{Operand, Offset, Register},
    }
};


pub fn setup_tracing(level: tracing::level_filters::LevelFilter) {
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_target(false)
        .with_line_number(true)
        .with_file(true)
        .with_thread_ids(false)
        .with_max_level(level)
        .without_time()
        .with_test_writer()
        .pretty()
        .finish();

    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).expect("Subscriber setup should succeed");
}


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
pub struct Disassembly {
    lines: Vec<Option<Line>>,
    pointer: Offset,
    width: usize
}
#[allow(unused)]
impl Disassembly {
    pub fn new(bytes: usize) -> Disassembly {
        let mut lines: Vec<Option<Line>> = Vec::with_capacity(bytes);
        for _ in 0..bytes { lines.push(None); }

        Disassembly {lines, ..Default::default() }
    }

    fn update<F>(&mut self, offset: Offset, update: F) -> Result<(), DecodeError>
        where F: FnOnce(&mut Line)
    { 
        let insert_line = self.lines.get_mut(offset.0 as usize);
        if insert_line.is_some() {
            let line = insert_line.unwrap();
            let l = line.get_or_insert_with(|| Line::default());
            update(l);
            Ok(())
        } 
        else { 
            Err(DecodeError::InvalidAddress(offset.0) )
        }
    }

    /// Add instruction to the Disassembly at the location pointed 
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
impl From<Vec<u8>> for Disassembly 
{
    fn from(bytes: Vec<u8>) -> Self {
        let mut output = Disassembly::new(bytes.len()); // Largest output is one line per byte
        let pointer = 0; // All streams start at zero
        let mut offset = Offset(pointer);


        while offset.to_pointer() < bytes.len() {
            let mut instruction = Bytes::Uknown(bytes[offset.to_pointer()]); // base case is byte is unknown

            let opcode_byte = &bytes[offset.to_pointer()];
            debug!("Found tentative OpCode: 0x{opcode_byte:X}");
            
            let Ok(rules) = DecodeRules::get(opcode_byte)
                else {
                    error!("Unexpected OpCode byte: 0x{opcode_byte:X}");
                    let _ = output.add(instruction.clone());
                    offset.increment(1 /*byte*/);
                    continue;
                }; 

            let instruction_idx = offset.to_pointer();
            debug!("Instruction Index: {instruction_idx}");

            for rule in rules { // We dont know which rule will decode into an instruction
                debug!("Trying rule: {rule}");
                let (min_length, _final) = rule.len();

                instruction = match rule.modrm_required() {
                    true => { 
                        debug!("ModRM required for instruction decode");

                        let modrm_idx: usize = rule.op_code().len();
                        let modrm_byte = bytes[instruction_idx + modrm_idx];
                        debug!("MODRM byte {modrm_byte} at location: {}", instruction_idx+modrm_idx);
                        let Ok(modrm) = rule.modrm_byte(modrm_byte) else { continue };
                        debug!("Got ModRM Byte: 0x{:X} = {modrm:?}", modrm.as_byte());

                        let sib = if modrm.precedes_sib_byte() {
                            let sib_byte = bytes[instruction_idx + modrm_idx+1];
                            debug!("Attempting decode of SIB byte from 0x{:X}", sib_byte);
                            let sib = Sib::try_from(sib_byte);
                            if sib.is_err() { continue; } 
                            else { sib.ok() }
                        } else { None };

                        // So, what does this do? It takes the minimum amount of bytes needed for
                        // the instruction ad indicated byte the decode rule and adds any bytes
                        // from MODRM or SIB byte displacement encodings.
                        let Ok(bytes_remaining) = modrm.bytes_remaining(sib)
                            else { 
                                debug!("Failed to determine remaining bytes from MODRM and SIB bytes");
                                continue;
                            };
                        let byte_range = offset.to_pointer().. offset.to_pointer() + min_length + bytes_remaining;


                        debug!("Grabbing byte range {:?} for decode attempt", byte_range);

                        let Some(prospective_bytes) = bytes.get(byte_range)
                               else { 
                                   error!("Test should have enough bytes for decoding instruction");
                                   continue
                               };

                        let decode_attempt = Bytes::from(offset.clone(), prospective_bytes, rule.clone());

                        let instruction = if decode_attempt.is_ok() { 
                            decode_attempt.expect("Ok due to conditional")
                        }
                        else { 
                            info!("Decode unsuccessful");
                            continue 
                        };
                        
                        instruction
                    },
                    false => { // know length a priori
                        let (length, _final) = rule.len();
                        let byte_range = instruction_idx..=instruction_idx+length;

                        debug!("Rule reported true byte length {length} for an instruction byte range of {byte_range:?}");

                        let Some(prospective_bytes) = bytes.get(instruction_idx.. instruction_idx + length)
                            else { 
                                error!("Attempted to grab more bytes than remain");
                                continue
                            };

                        debug!("Tentative Bytes: {}",prospective_bytes.iter().map(|c| format!("0x{c:X}  ")).collect::<String>());

                        let decode_attempt = Bytes::from(offset.clone(), prospective_bytes, rule.clone());

                        let instruction = if decode_attempt.is_ok() {
                            debug!("Successfully decoded: {decode_attempt:?}");
                            let instruction = decode_attempt.expect("Ok due to conditional");
                            instruction
                        }
                        else { 
                            info!("Decode unsuccessful");
                            continue
                        };
                        instruction
                    },
                };

                if rule.can_make_label() { // Unknown bytes make no labels.
                    let instruction = instruction
                        .get_instruction().expect("Should be a valid instruction");

                    let has_label_operand = instruction.operands.iter().any(|op| matches!(op, Operand::Label(_)));

                    if has_label_operand {
                        let label = instruction
                            .get_displacement_offset().expect("Should have label");

                        let _ = output.label(label); // Dont worry about the result in a test. We'll
                                                     // regularly add labels "beyond" range
                    }
                }
                if instruction.decoded_successfully() { break }
            }
            // We have either decoded an instruction or we have an unknown byte. 
            // In any case, we add it to the disasembly output.
            offset.increment(instruction.length() as u32);
            let _ = output.add(instruction.clone()); 

        };
        output
    }
}
impl Display for Disassembly {
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

#[test]
fn single_line() {
    let expected = vec![
        "00000000: 74 0F     jz offset_00000018h"
    ].join("\n");
    let mut output = Disassembly::new(11);

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
    let mut output = Disassembly::new(3);

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
    

    let mut output = Disassembly::new(11);

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

    let mut output = Disassembly::new(11);

    let _ = output.add(jz);
    let _ = output.label(Offset(0x2));
    let _ = output.add(add);
    assert_eq!(format!("{output}"), expected);
}
