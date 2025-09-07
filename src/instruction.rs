use crate::decode::DecodeError;
use encoding::{
    Displacement, Immediate,
};
use memory::{Register, Memory};


/// Possible Operand Encodings that are used to construct the
/// the instruction decode rules. This is an intermediate type.
#[allow(unused)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OpEn {
    RM, MR, MI,  M, I, 
    NP, ZO,  O, OI, D,
    
    /// Treat Moffs as Imm32
    FD, TD,
} 
#[allow(unused)]
impl OpEn {
    pub fn modrm_required(&self) -> bool {
        match self {
            OpEn::RM | OpEn::MR | OpEn::MI | OpEn::M => true,
            _ => false,
        }
    }

}

/// OperandEncoding as described by the Intel instruction manual
/// for the required instructions.
#[allow(unused)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum OperandEncoding {
    /// ModRM:reg & ModRM:r/m
    RM {register: Register, memory: Memory },

    /// ModRM:r/m & ModRM:reg
    MR { memory: Memory, register: Register },

    /// ModRM:r/m & imm8/16/32
    MI { memory: Memory, immediate: Immediate },

    /// ModRM:r/m  
    M  { memory: Memory },

    /// imm8/16/32
    /// May also have implied register such as `add eax imm32`
    I(Immediate),

    /// NoOp
    NP,

    /// Zero Operands
    ZO,

    /// Add register number to the Opcode
    O,

    /// 1st: Add register number to Opcode
    /// 2nd: Immediate
    OI(Immediate),

    /// Relative Displacement
    D(Displacement),

    /// Treat Moffs as Imm32
    FD, TD
}

pub mod memory {
    use super::*;

    #[allow(unused)]
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub enum Register {
        EAX = 0, ECX = 1, EDX = 2, EBX = 3,
        ESP = 4, EBP = 5, ESI = 6, EDI = 7,
    } 
    impl TryFrom<u8> for Register {
        type Error = DecodeError;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Register::EAX),
                1 => Ok(Register::ECX),
                2 => Ok(Register::EDX),
                3 => Ok(Register::EBX),
                4 => Ok(Register::ESP),
                5 => Ok(Register::EBP),
                6 => Ok(Register::ESI),
                7 => Ok(Register::EDI),
                _ => Err(DecodeError::InvalidRegister),
            }
        }
    }
    impl Into<&str> for Register {
        fn into(self) -> &'static str {
            match self {
                Register::EAX => "eax",
                Register::ECX => "ecx",
                Register::EDX => "edx",
                Register::EBX => "ebx",
                Register::ESP => "esp",
                Register::EBP => "ebp",
                Register::ESI => "esi",
                Register::EDI => "edi",
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct Memory {}
}

pub mod encoding {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct Prefix(pub u8);

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct OpCode(pub &'static [u8]);

    pub mod extensions {
        use super::*;

        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub struct ExtSet(pub &'static [&'static str]);

        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub enum Extension {
            RW, RD,
            IB, IW, ID,
            SR,
            S0, S1, S2, S3, S4, S5, S6, S7

        }
        impl TryFrom<&'static str> for Extension {
            type Error = DecodeError;

            fn try_from(value: &'static str) -> Result<Self, Self::Error> {
                match value {
                    "+rw" => Ok(Extension::RW),
                    "+rd" => Ok(Extension::RD),
                    "ib"  => Ok(Extension::IB),
                    "iw"  => Ok(Extension::IW),
                    "id"  => Ok(Extension::ID),
                    "/r"  => Ok(Extension::SR),
                    "/0"  => Ok(Extension::S0),
                    "/1"  => Ok(Extension::S1),
                    "/2"  => Ok(Extension::S2),
                    "/3"  => Ok(Extension::S3),
                    "/4"  => Ok(Extension::S4),
                    "/5"  => Ok(Extension::S5),
                    "/6"  => Ok(Extension::S6),
                    "/7"  => Ok(Extension::S7),
                    _ => Err(DecodeError::InvalidOpCodeExtension)
                }
            }
        }
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
    pub struct AddressingModes(pub &'static [u8]);

    #[allow(unused)]
    #[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
    pub struct ModRM (
        /// MOD
        /// Only low two bits are valid
        pub u8,
        /// REG
        /// Only low three bits are valid
        pub u8,
        // RM
        // Only low three bits are valid
        pub u8
    ); 
    impl ModRM {
        pub const fn _len() -> usize { 1 }

        #[allow(unused)]
        fn syntax(&self) -> Result<String,DecodeError>  {
            match self.0 {
                0b00 => {
                    Ok("todo".to_string())
                },
                0b01 => { // [r/m + byte]
                    Ok("todo".to_string())
                },
                0b10 => { // [r/m + dword] 
                    Ok("todo".to_string())
                },
                0b11 => { // r/m

                    Ok("todo".to_string())
                }
                _ => Err(DecodeError::InvalidAddressingMode),
            }
        }
    }
    impl From<u8> for ModRM {
        fn from(value: u8) -> Self {
            Self (
                (value & 0b11000000) >> 6,
                (value & 0b00111000) >> 3,
                (value & 0b00000111) >> 0,
            )
        }
    }

    #[allow(unused)]
    #[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
    pub struct Sib (
        /// Scale
        // Only low two bits are valid
        u8,
        /// Index
        // Only low three bits are valid
        u8,
        /// Base
        // Only low three bits are valid
        u8
    ); 
    impl Sib {
        pub const fn _len() -> usize { 1 }

        /// Returns the format str that can be used to print an instruction. 
        fn _format_string(&self, modrm: ModRM) -> &str {
            let special_case = self.2 == 0b101 && modrm.0 == 0b00;
            match self.0 {
                0b00 => { "[ {} + {}]" },
                0b01 => { 
                    if special_case {        // [ indexreg*2 + displacement ] 
                        "[ {}*2 + {} ]"      
                    } else {                 // [ indexreg*2 + basereg + displacement]
                        "[ {}*2 + {} + {} ]"
                    }
                },
                0b10 => { 
                    if special_case {        // [ indexreg*4 + displacement ] 
                        " [ {}*4 + {} ]"
                    } else {                 // [ indexreg*4 + basereg + displacement ]
                        "[ {}*4 + {} + {} ]"
                    }
                },
                0b11 => { 
                    if special_case {        // [ indexreg*8 + displacement ]
                        "[ {}*8 + {}]"
                    } else {                 // [ indexreg*8 + basereg + displacement ]
                        "[ {}*8 + {} + {} ]"
                    }
                },
                _ => unreachable!("This should never happen")
            }
        }
    }
    impl From<u8> for Sib {
        fn from(value: u8) -> Self {
            Self (
                (value & 0b11000000) >> 6,
                (value & 0b00111000) >> 3,
                (value & 0b00000111) >> 0,
            )
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
    pub struct Displacement(u32);
    impl Displacement {
        /// The number of bytes contained in the Displacement
        /// as seen on disk or in a file
        pub fn _len(&self) -> usize {
            unimplemented!("TODO")
        }
    }

    #[allow(unused)]
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub enum Immediate {
        Imm8(u8),
        Imm16(u16),
        Imm32(u32),
        Imm64(u64),
    }
}
