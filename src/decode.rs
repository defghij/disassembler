use bitmask_enum::bitmask;

use crate::opcodes::presumptive_decode_rules;
use crate::instruction::{
    OpEn,                
    memory::{Register, Memory},
    encoding::{
        Prefix, OpCode, ModRM, Sib, Displacement, Immediate,
        AddressingModes,
        extensions::{ExtSet, Extension},
    }
};

fn simple_test() {
    let cmp_ecx_edx: [u8;2] = [0x39, 0xD1];
    let first_byte = cmp_ecx_edx.get(0).unwrap();
    let dc_rule = presumptive_decode_rules(*first_byte);
    assert!(dc_rule.len() == 1);
    //if dc_rule.modrm_required() {
        //let modrm: Modrm = cmp_ecx_edx[1..2].try_from();
    //}
}


#[allow(unused)]
#[derive(Clone, Debug)]
pub enum DecodeError {
    InvalidAddressingMode,
    IllegalAddressMode,
    InvalidOpCodeExtension,
    InvalidRegister,
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
impl DecodeRule { 

    /// Returns the length, in bytes, of the instruction
    /// that the rule encodes
    fn len(&self) -> usize {
        let mut len: usize = 0; 
        if self.1.is_some() { len += 1; }
        //len += self.2.len();
        
        unimplemented!("How do?")
    }

    fn extensions(&self) -> Option<Vec<Extension>> {
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

    fn modrm_required(&self) -> bool {
        let op_enc = self.4.as_ref();
        match op_enc {
            Some(op_enc) =>  op_enc.modrm_required(),
            None => false,
        }

    }
}
impl TryFrom<String> for DecodeRule {
    type Error = DecodeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        unimplemented!("Still more work to do")
    }
}
impl TryFrom<&[u8]> for DecodeRule {
    type Error = DecodeError;

    fn try_from(_value: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!("Still more work to do")
    }
}
