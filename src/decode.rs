use std::fmt::Display;

#[allow(unused_imports)]

use bitmask_enum::bitmask;

#[allow(unused)]
use crate::{
    opcodes::DecodeRules,
    instruction::{
        OpEn,                
        memory::{
            Register, Memory},
        encoding::{
            Prefix, OpCode, ModRM, Sib, Displacement, Immediate,
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
        instruction: String,
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

    pub fn string(&self) -> String {
        match self {
            Bytes::Decoded { bytes: _ , instruction } => instruction.clone(),
            Bytes::Uknown(b) | Bytes::Illegal(b)  => format!("db 0x{b:02X}"),
            Bytes::None => "".into()
        }
        //self.instruction.clone()
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

    pub fn from(bytes: &[u8], rule: DecodeRule) -> Bytes {
        if bytes.len() == 0 { return Bytes::None }

        let (mnemonic, prefix, op_code, extensions, op_encode, addr_modes) = rule.separate();

        match op_encode {
            OpEn::O => {
                let byte = bytes[0];

                // All Single byte OpEn::O instructions _should have one and only one extension
                // "/rd"
                if extensions.is_some_and(|ext| ext.len() ==1 && ext[0] == Extension::RD) { 
                    let opcode: u8 = op_code.0[0]; // Single byte Opcode by virtue of being in this
                                                   // function
                    let reg_value = byte - opcode;

                    let register = Register::try_from(reg_value)
                        .expect("Opcde and Byte should be within the register range");
                    let instruction = format!("{mnemonic} {register}");
                    Bytes::Decoded {
                        bytes: vec![byte],
                        instruction
                    }

                }
                else { Bytes::Uknown(byte) }
            },
            OpEn::ZO => {
                assert!(extensions.is_none());
                Bytes::Decoded {
                    bytes: vec![bytes[0]],
                    instruction: mnemonic.to_string()
                }
            },
            OpEn::I => {
                // Validate instruction assumptions.
                if op_code.len() != 1   { return Bytes::Uknown(bytes[0]); }
                if extensions.is_none() { return Bytes::Uknown(bytes[0]); }

                let extensions = extensions.expect("Should be Some due to conditional above");
                if extensions.len() != 1 { return Bytes::Uknown(bytes[0]); }

                match extensions[0] {
                    Extension::ID => {
                        if bytes.len() != 5 { Bytes::Uknown(bytes[0]) }
                        else {
                            let imm = Immediate::Imm32(bytes[1..5].to_vec());

                            Bytes::Decoded {
                                bytes: bytes.to_vec(),
                                instruction: format!("{rule} {imm}")
                            }
                        }
                    },
                    Extension::IW => {
                        println!("w");
                        unimplemented!()
                    },
                    Extension::IB => {
                        let imm = Immediate::Imm8(bytes[1..2].to_vec());

                        Bytes::Decoded {
                            bytes: bytes.to_vec(),
                            instruction: format!("{rule} {imm}")
                        }
                    },
                    _ => Bytes::Uknown(bytes[0])
                }
            }
            _ => Bytes::Uknown(bytes[0]),
        }

    }

    pub fn operands(&self) -> Option<Vec<String>> { unimplemented!("lol"); }
    pub fn mnemonic(&self) -> Option<String> { unimplemented!("lol"); }
    pub fn prefix(&self) -> Option<String> { unimplemented!("lol"); }
}
impl Default for Bytes {
    fn default() -> Self { Bytes::None }
}


#[allow(unused)]
#[derive(Clone, Debug, PartialEq, Hash)]
pub enum DecodeError {
    NoBytesPresent,
    UnknownOpcode,
    InvalidAddressingMode,
    IllegalAddressMode,
    InvalidOpCodeExtension,
    InvalidRegister,
    InvalidAddress(u32),
    AddressConflict(u32),
    InvalidImmediateSize(usize)
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
    /// Returns the length, in bytes, of the instruction
    /// that the rule encodes
    pub fn len(&self) -> usize {
        let (mnemonic, prefix, op_code, extensions, op_encoding, addr_modes) = self.separate();
        
        if op_code.len() == 1 && op_encoding.operand_count() == 0 {
            return 1;
        }

        // We have a single immediate operand. Extension will encode the operand length.
        if op_encoding == OpEn::I {
            let extensions = extensions.expect("All Rules with an OpEn::I should require an extension");

            if extensions.len() == 1 {
                let ext = extensions[0].clone();
                let bytes = ext.operand_length().expect("Extension should encode operand length for OpEn::I");
                return op_code.len() + bytes
            }
        }



        let mut len: usize = 0; 
        if self.1.is_some() { len += 1; }
        //len += self.2.len();
        
        unimplemented!("How do?")
    }

    pub fn modrm_required(&self) -> bool {
        self.4.modrm_required()
    }

    pub fn implicit_operand(&self) -> Option<&'static str> {
        let (mnemonic, prefix, op_code, extensions, op_encoding, addr_modes) = self.separate();
        if op_encoding == OpEn::I && extensions.is_some() {
            let extensions = extensions.expect("Should be Some by virtue of above conditional");

            match op_code.bytes()[0] {
                // OpCodes where an operands implied by the OpCode
                0x2D | 0x05 => {
                    if extensions.contains(&Extension::IB) { return Some("al"); } else
                    if extensions.contains(&Extension::IW) { return Some("ax"); } else
                    if extensions.contains(&Extension::ID) { return Some("eax"); } 
                    else { return None; }
                },
                _ => None
            }
        }
        else { None }
    }

    pub fn mnemonic(&self) -> String { self.0.to_string() }
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

