use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum DecodeError {
    DecodeFailure,
    NoBytesPresent,
    UnknownOpcode,
    InvalidModRM,
    InvalidLength,
    InvalidAddressingMode,
    InvalidOpCodeExtension,
    InvalidOpCodeLength,
    InvalidRegister,
    InvalidAddress(u32),
    InvalidImmediateSize(usize),
    InvalidDisplacementByteWidth,
    _IllegalAddressMode,
    _AddressConflict(u32),
}

use crate::instruction::encoding::operands::Offset;
#[allow(unused)]
use crate::{
    opcodes::DecodeRules,
    instruction::{
        Instruction,
        OpEn,                
        memory::Memory,
        encoding::{
            operands::{
                Operand, Displacement, Immediate, Register
            },
            Prefix, OpCode, ModRM, Sib,
            AddressingModes,
            extensions::{ExtSet, Extension},
        }
    }
};

#[allow(unused)]
#[derive(Clone, Debug, PartialEq)]
pub enum Bytes {
    /// Bytes representing a decoded instruction.
    Decoded { 
        bytes: Vec<u8>,
        instruction: Instruction,
    },
    /// An unknown byte or opcode
    Uknown(u8),
    /// An illegal instruction. Currently, only a single bytes.
    Illegal(u8),
    None
}
#[allow(unused)]
impl Bytes {
    pub fn decoded_successfully(&self) -> bool {
        match self {
            Bytes::Uknown(_) | Bytes::Illegal(_) | Bytes::None => false,
            _ => true
        }
    }

    pub fn get_instruction(&self) -> Option<Instruction> {
        match self {
            Bytes::Decoded { bytes, instruction } => Some(instruction.clone()),
            _ => None
        }
    }

    pub fn get_bytes(&self) -> Option<Instruction> {
        match self {
            Bytes::Decoded { bytes, instruction } => Some(instruction.clone()),
            _ => None
        }
    }

    fn raw_bytes(&self) -> Vec<u8> {
        match self {
            Bytes::Decoded { bytes, instruction: _ } => bytes.to_vec(),
            Bytes::Uknown(byte) | Bytes::Illegal(byte) => vec![byte.clone()],
            Bytes::None => Vec::new()
        }
    }

    pub fn bytes(&self) -> String {
        self.raw_bytes()
            .iter().map(|b| format!("{b:02X}") )
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn length(&self) -> usize {
        self.raw_bytes().len()
    }

    pub fn from(location: Offset, bytes: &[u8], rule: DecodeRule) -> Result<Bytes, DecodeError> {
        if bytes.len() == 0 { return Err(DecodeError::NoBytesPresent); }
        let invalid = Bytes::Uknown(bytes[0]);

        let (mnemonic, prefix, op_code, extensions, op_encode, addr_modes) = rule.separate();

        let byte = bytes[0];
        let opcode: u8 = op_code.0[0];
        let (instruction_length, fixed) = rule.len();
        let opcode_length = rule.op_code().len();
        let mut instruction = Instruction::new(mnemonic);

        match op_encode {
            OpEn::O  => {

                // All Single byte OpEn::O instructions _should have one and only one extension
                // "/rd"
                if extensions.is_some_and(|ext| ext.len() ==1 && ext[0] == Extension::RD) { 
                    let reg_value = byte - opcode;

                    let Ok(register) = Register::try_from(reg_value)
                    else { return Err(DecodeError::InvalidRegister); };

                    instruction.add(Operand::Register(register));

                    Ok(Bytes::Decoded {
                        bytes: vec![byte],
                        instruction: instruction.clone()
                    })

                }
                else { Err(DecodeError::DecodeFailure) }
            },
            OpEn::ZO => {
                if extensions.is_none() { 
                    Ok(Bytes::Decoded {
                        bytes: vec![bytes[0]],
                        instruction: instruction.clone()
                    })
                }
                else { return Err(DecodeError::InvalidOpCodeExtension); } 
            },
            OpEn::I  => {
                // Validate instruction assumptions.
                if op_code.len() != 1   { return Err(DecodeError::InvalidOpCodeLength); }
                if extensions.is_none() { return Err(DecodeError::InvalidOpCodeExtension); }
                let extensions = extensions.unwrap();
                if extensions.len() != 1 { return Err(DecodeError::InvalidOpCodeExtension); }

                if rule.implicit_operand().is_some() {
                    let register = rule.implicit_operand().unwrap();
                    instruction.add(Operand::Register(register));
                }

                if extensions.contains(&Extension::IB) { 
                    let imm = Immediate::Imm8(bytes[1..2].to_vec());
                    instruction.add(Operand::Immediate(imm));
                }

                if extensions.contains(&Extension::IW) { 
                    unimplemented!("Opcode Extension for immediate word is not implemented")
                }

                if extensions.contains(&Extension::ID) { 
                    let imm = Immediate::Imm32(bytes[1..5].to_vec());
                    instruction.add(Operand::Immediate(imm));
                }

                Ok(Bytes::Decoded {
                    bytes: bytes.to_vec(),
                    instruction: instruction.clone()
                })
            }
            OpEn::D  => {
                let displacement_length = instruction_length - opcode_length;

                let range = (opcode_length.. opcode_length + displacement_length);

                let displacement_bytes = bytes.get(range).expect("Displacement range should be correct");

                let displacement = match displacement_length {
                    1 => {
                        let operands = <[u8;1]>::try_from(displacement_bytes).expect("Displacement length calculation should be correct");
                        Displacement::from_byte_relative(location,opcode_length, &operands)
                    },
                    2 => {
                        let operands = <[u8;2]>::try_from(displacement_bytes).expect("Displacement length calculation should be correct");
                        Displacement::from_word_relative(location,opcode_length, &operands)
                    }, 
                    4 => {
                        let operands = <[u8;4]>::try_from(displacement_bytes).expect("Displacement length calculation should be correct");
                        println!("Operands: {}", operands.iter().map(|b| format!("{b:X}")).collect::<Vec<String>>().join(" "));
                        Displacement::from_double_relative(location,opcode_length, &operands)
                    },
                    _ => return Err(DecodeError::InvalidDisplacementByteWidth),
                };
                let mut instruction = Instruction::new(mnemonic);
                instruction.add(Operand::Displacement(displacement));

                Ok(Bytes::Decoded {
                    bytes: bytes.to_vec(),
                    instruction: instruction.clone(),
                })
            }
            OpEn::OI => {
                let Some(extensions) = extensions 
                else { return Err(DecodeError::InvalidOpCodeExtension); };

                let mut instruction = Instruction::new(mnemonic);

                if extensions.contains(&Extension::RD) {
                    let reg_value = byte - opcode;

                    let register = Register::try_from(reg_value)
                        .expect("Opcde and Byte should be within the register range");

                    instruction.add(Operand::Register(register));
                }

                if extensions.contains(&Extension::IB) { 
                    let imm = Immediate::Imm8(bytes[1..2].to_vec());
                    instruction.add(Operand::Immediate(imm));
                }

                if extensions.contains(&Extension::IW) { 
                    unimplemented!("Opcode Extension for immediate word is not implemented")
                }

                if extensions.contains(&Extension::ID) { 
                    let imm = Immediate::Imm32(bytes[1..5].to_vec());
                    instruction.add(Operand::Immediate(imm));
                }

                Ok(Bytes::Decoded {
                    bytes: bytes.to_vec(),
                    instruction: instruction.clone()
                })
            }
            OpEn::M => { 
                if rule.modrm_required() && instruction_length == 1 { return Err(DecodeError::DecodeFailure); }

                // All declared OpEn::M `DecodeRules` have an extension.
                let Some(extensions) = extensions 
                else { return Err(DecodeError::InvalidOpCodeExtension)};


                let Some(modrm) = rule.modrm_byte(bytes[opcode_length])
                else { return Err(DecodeError::InvalidModRM); };
                    
                if !addr_modes.is_some_and(|a| a.0.contains(&modrm.0)) { 
                    return Err(DecodeError::InvalidAddressingMode); 
                }

                let mut instruction = Instruction::new(mnemonic);

                match modrm.0 {
                    0b00 => {
                        match modrm.2 {
                            0b100 => { unimplemented!("SIB byte not implemented for this address mode") },
                            0b101 => { 
                                let base: usize = opcode_length + 1/*modrm byte*/;

                                // This should probably really be a panic as it means a fundamental
                                // flaw in parsing logic.
                                if bytes.len() < base + 4 { return Err(DecodeError::InvalidLength); }

                                let displacement: &[u8] = bytes.get(base..base+4).expect("Length bounds should be correct due to assert");

                                let Ok(displacement) = <[u8;4]>::try_from(displacement) 
                                else { return Err(DecodeError::DecodeFailure); };

                                //let displacement = Displacement::from_double_absolute(&displacement);
                                //instruction.add(Operand::Displacement(displacement));
                                unimplemented!("Need to implement EffectiveAddress output for [ disp32 ] first");
                            },
                            _     => { 
                                let Ok(register) = Register::try_from(modrm.2)
                                else { return Err(DecodeError::InvalidRegister); };
                                //let effective_address = EffectiveAddress::from();?
                                //instruction.add(Operand::EffectiveAddress(register));
                                unimplemented!("Need to implement EffectiveAddress output for [ reg ] first");
                            }

                        }
                    },
                    0b01 => {unimplemented!("Addressing mode not implemented")},
                    0b10 => {unimplemented!("Addressing mode not implemented")},
                    0b11 => {
                        let Ok(register) = Register::try_from(modrm.2)
                        else { return Err(DecodeError::InvalidRegister); };

                        instruction.add(Operand::Register(register));
                    }
                    _ => return Err(DecodeError::InvalidAddressingMode)
                }

                Ok(Bytes::Decoded {
                    bytes: bytes.to_vec(),
                    instruction: instruction.clone()
                })
            },
            OpEn::RM => todo!(),
            OpEn::MR => todo!(),
            OpEn::MI => todo!(),
            OpEn::NP => todo!(),
            OpEn::FD => todo!(),
            OpEn::TD => todo!(),
        }

    }

    pub fn operands(&self) -> Option<Vec<String>> { unimplemented!("lol"); }
    pub fn mnemonic(&self) -> Option<String> { unimplemented!("not yet"); }
    pub fn prefix(&self) -> Option<String> { unimplemented!("lol"); }
}
impl Default for Bytes {
    fn default() -> Self { Bytes::None }
}
impl Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Bytes::Decoded { bytes: _ , instruction } => format!("{instruction}"),
            Bytes::Uknown(b) | Bytes::Illegal(b)  => format!("db 0x{b:02X}"),
            Bytes::None => "".into()
        };
        write!(f, "{string}")
    }
}



/// This structure attempts to encapsulates all the information
/// the application may need when attempting to determine the 
/// whether the byte(s) represent a valid instruction.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DecodeRule(
    pub &'static str,             // 0
    pub Option<Prefix>,           // 1
    pub OpCode,                   // 2
    pub Option<ExtSet>,           // 3
    pub OpEn,                     // 4
    pub Option<AddressingModes>,  // 5
);

#[allow(unused)]
impl DecodeRule { 
    pub fn separate(&self) -> (&'static str, Option<Prefix>, OpCode, Option<Vec<Extension>>, OpEn, Option<AddressingModes>) {
        ( self.0, self.1.clone(), self.2.clone(), self.extensions(), self.4.clone(), self.5.clone())
    }

    /// Returns a minimum length that is needed to encode the instruction and a bool indicating
    /// whether the value yielded is definitive.
    ///
    /// A return value of (2, true), means the true length of the instruction is 2. A return result
    /// of (2, false) means that this is a minimum length of the instruction and follow on
    /// processing will likely be needed to determine the actual length of the instruction byte
    /// stream.
    pub fn len(&self) -> (usize, bool) {
        let (mnemonic, prefix, op_code, extensions, op_encoding, addr_modes) = self.separate();
        
        if op_code.len() == 1 && op_encoding.operand_count() == 0 {
            return (1,true);
        }

        // This match statement currently has a lot of duplicated code. If iterating over extension
        // operand length turns out to be sufficient this can be reduced/removed.
        match op_encoding {
            OpEn::I | OpEn::OI | OpEn::D => {
                let extensions = extensions.as_ref().expect("All rules in this match statement should require an extension");
                let mut bytes = extensions.iter()
                    .filter(|ext| ext.operand_length().is_some())
                    .fold(0, |acc, ext| acc + ext.operand_length().expect("Should be some due to fiter") );

                if self.modrm_required() { bytes += 1 }
                (op_code.len() + bytes, true)
            },
            OpEn::RM => unimplemented!("`len` not implemented for this Operand Encoding"),
            OpEn::MR => unimplemented!("`len` not implemented for this Operand Encoding"),
            OpEn::MI => unimplemented!("`len` not implemented for this Operand Encoding"),
            OpEn::M => {
                let extensions = extensions.as_ref().expect("All Rules with an OpEn::M encoding should require an extension");
                let mut bytes = extensions.iter()
                    .filter(|ext| ext.operand_length().is_some())
                    .fold(0, |acc, ext| acc + ext.operand_length().expect("Should be some due to fiter") );

                if self.modrm_required() { bytes += 1 }
                (op_code.len() + bytes, false)
            },
            OpEn::NP => unimplemented!("`len` not implemented for this Operand Encoding"),
            OpEn::ZO => unimplemented!("`len` not implemented for this Operand Encoding"),
            OpEn::ZO => unimplemented!("`len` not implemented for this Operand Encoding"),
            OpEn::O  => unimplemented!("`len` not implemented for this Operand Encoding"),
            OpEn::FD => unimplemented!("`len` not implemented for this Operand Encoding"),
            OpEn::TD => unimplemented!("`len` not implemented for this Operand Encoding"),
        }

        //let mut len: usize = 0; 
        //if self.1.is_some() { len += 1; }
        //len += self.2.len();
        
        //unimplemented!("How do?")
    }

    pub fn modrm_required(&self) -> bool {
        self.4.modrm_required()
    }

    /// Takes a [u8] and yields a Some([ModRM]) if the byte can be validated as a ModRM byte for the
    /// particular [DecodeRule] that self describes. If is not valid, then a [None] is returned.
    pub fn modrm_byte(&self, byte: u8) -> Option<ModRM> {
        let ext_set      = self.3.clone();
        let addr_mode    = self.5.clone(); 
        let modrm        = ModRM::from(byte); 
        let (md, rg, rm) = modrm.split();

        // Check that if extension dictates a value in the modrm byte that it is set.
        if ext_set.is_some() {
            let extensions = ext_set.as_ref().expect("Should be some due to conditional");
            let Some(sdigit) = extensions.get_sdigit() else { return None };

            // Decoding rule and ModRM have incompatible REG bits.
            if !sdigit.is_sdigit(rg) { 
                return None; }
        }


        // Check that addressing mode is valid for this rule
        if addr_mode.is_some() {
            let addressing_mode = addr_mode.as_ref().expect("Should be some due to conditional");

            // Decoding rule and ModRM have incompatible MOD bits
            if !addressing_mode.0.contains(&md) { return None; }
        }
        Some(modrm)
    }

    pub fn implicit_operand(&self) -> Option<Register> {
        let (_, _, op_code, extensions, op_encoding, _) = self.separate();
        if op_encoding == OpEn::I && extensions.is_some() {
            let extensions = extensions.expect("Should be Some by virtue of above conditional");

            match op_code.bytes()[0] {
                // OpCodes where an operands implied by the OpCode
                0x2D | 0x05 => {
                    if extensions.contains(&Extension::IB) { return Some(Register::AL); } else
                    if extensions.contains(&Extension::IW) { return Some(Register::AX); } else
                    if extensions.contains(&Extension::ID) { return Some(Register::EAX); } 
                    else { return None; }
                },
                _ => None
            }
        }
        else { None }
    }

    pub fn makes_label(&self) -> bool {
        match self.mnemonic() {
            "call" => true,
            _ => false,
        }
    }

    pub fn mnemonic(&self) -> &'static str { self.0 }
    pub fn op_code(&self) -> OpCode { self.2.clone() }
    pub fn extensions(&self) -> Option<Vec<Extension>> { 
        match &self.3 {
            None => None,
            Some(ext_set) => {
                let extensions = ext_set.0
                    .iter()
                    .map(|ext| { 
                        Extension::try_from(*ext)
                            .expect("Extensions should be hardcoded and always valid")
                    })
                    .collect::<Vec<Extension>>();
                Some(extensions)
            },
        }
    }
    pub fn op_encoding(&self) -> OpEn { self.4.clone() }
    pub fn address_modes(&self) -> Option<AddressingModes> { self.5.clone() }
}
impl Display for DecodeRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mnemonic = self.mnemonic();
    
        let implicit_operand = if self.implicit_operand().is_some() {
            self.implicit_operand().expect("Should be some by virtue of previous clasue").to_string()
        } else { "".to_string() };

        let out = if implicit_operand.is_empty() { 
            format!("{}", mnemonic)
        } else { format!("{mnemonic} {implicit_operand},") };

        write!(f, "{out}")
    }
}

