#[allow(unused_imports)]

use bitmask_enum::bitmask;

#[allow(unused)]
use crate::{
    //opcodes::DecodeRules,
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
    InvalidAddressingMode,
    IllegalAddressMode,
    InvalidOpCodeExtension,
    InvalidRegister,
    InvalidAddress(u32),
    AddressConflict(u32),
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
    pub Option<OpEn>,             // 4
    pub Option<AddressingModes>,  // 5
);

#[allow(unused)]
impl DecodeRule { 

    /// Returns the length, in bytes, of the instruction
    /// that the rule encodes
    pub fn len(&self) -> usize {
        let mut len: usize = 0; 
        if self.1.is_some() { len += 1; }
        //len += self.2.len();
        
        unimplemented!("How do?")
    }

    pub fn extensions(&self) -> Option<Vec<Extension>> {
        let ext_set = self.3.as_ref();
        match ext_set {
            Some(raw) => {
                //let ext: Vec<Extension> = raw.iter().map(|x| {
                    //let ext: Extension = x.try_into()
                        //.expect("Developer should have entered extensions correctly");
                    //ext
                //}).collect();
                //Ok(ext)
                unimplemented!("todo");
            },
            None => None,
        }
    }

    pub fn modrm_required(&self) -> bool {
        let op_enc = self.4.as_ref();
        match op_enc {
            Some(op_enc) =>  op_enc.modrm_required(),
            None => false,
        }

    }
}
impl TryFrom<String> for DecodeRule {
    type Error = DecodeError;

    fn try_from(_value: String) -> Result<Self, Self::Error> {
        unimplemented!("Still more work to do")
    }
}
impl TryFrom<&[u8]> for DecodeRule {
    type Error = DecodeError;

    fn try_from(_value: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!("Still more work to do")
    }
}
