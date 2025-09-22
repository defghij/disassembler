use std::fmt::Display;

use tracing::{debug, error};

use crate::decode::DecodeError;
use encoding::operands::{
    Offset, 
    Operand, 
    //Displacement, 
    //Immediate, 
    //Register
};

#[derive(Clone, Debug, PartialEq)]
pub struct Instruction {
    pub prefix: Option<String>,
    pub mnemonic: &'static str,
    pub operands: Vec<Operand>,
} 
impl Instruction {
    pub fn new(mnemonic: &'static str) -> Instruction {
        let mut i = Instruction::default();
        i.mnemonic = mnemonic;
        i
    }

    pub fn add(&mut self, op: Operand) -> &mut Self {
        self.operands.push(op);
        self
    }

    pub fn _update_prefix(&mut self, prefix: String) -> &mut Self {
        self.prefix = Some(prefix);
        self
    }

    /// Returns the offset of the instruction's reference.
    ///
    /// This function is for those instruction that reference (i.e.call or jump)
    /// to other locations/addresses that necessitate creating a label. 
    ///
    /// Specifically, this is only for instructions with the OpEn::D encoding.
    /// These will have only one operand.
    #[allow(unused)] // Currently, only used in tests
    pub fn get_displacement_offset(&self) -> Option<Offset> {
        let offsets: Vec<Offset> = self.operands
            .iter()
            .filter(|o| { matches!(o, Operand::Displacement(_)) || matches!(o, Operand::Label(_)) })
            .map(|o: &Operand| {
                let offset = match o {
                    Operand::Displacement(displacement) => displacement.clone().into(),
                    Operand::Label(offset) => (*offset).clone(),
                    _ => panic!("Should be unreachable due to the filter")
                };
                offset 
            })
        .collect();
        if offsets.len() == 1 {
            Some(offsets[0].clone())
        } else { None }
    }

    #[allow(unused)] // Currently, not used. Remove in future if unused.
    fn convert_displacements_to_offsets(&self) -> bool {
        match self.mnemonic {
            "call" | "jmp" | "jz" | "jnz" | "jne" => true,
            _ => false,
        }
    }
}
impl Default for Instruction {
    fn default() -> Self {
        Instruction {
            prefix: None,
            mnemonic: "",
            operands: Vec::new(),
        }
    }

}
impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = &self.prefix;
        let operands = &self.operands;

        let mut out = if prefix.is_some() {
            format!("{} ", self.prefix.clone()
                .expect("should be Some by virtue of the conditional"))
        } else { String::new() };
        out = format!("{out}{}", self.mnemonic);
        out = if !operands.is_empty() {
            let operands = operands
                .iter()
                .map(|o| {
                    o.to_string()
                })
                .collect::<Vec<String>>()
                .join(", ");
            format!("{out} {operands}")
        }
        else { out };
        
        write!(f, "{out}")
    }
}


/// Possible Operand Encodings that are used to construct the
/// the instruction decode rules. This is an intermediate type.
#[allow(unused)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OpEn {
    RM, MR, MI,  M, I, 
    NP, ZO,  O, OI, D,
    // Treat Moffs as Imm32
    FD, TD,
    // Part of Tests, not assignment
    M1, RMI
} 
#[allow(unused)]
impl OpEn {
    pub fn modrm_required(&self) -> bool {
        match self {
            OpEn::RM | OpEn::MR | OpEn::MI | OpEn::M | OpEn::M1 | OpEn::RMI => true,
            _ => false,
        }
    }

    pub fn operand_count(&self) -> usize {
        match self {
            OpEn::RM => 2,
            OpEn::MR => 2,
            OpEn::MI => 2,
            OpEn::M1 => 2,
            OpEn::M  => 1,
            OpEn::I  => 1,
            OpEn::ZO => 0,
            OpEn::O  => 0,
            OpEn::OI => 2,
            OpEn::D  => 1 ,
            OpEn::RMI  => 3,
            OpEn::NP => unimplemented!("Not part of the assignment"),
            OpEn::FD => unimplemented!("Not part of the assignment"),
            OpEn::TD => unimplemented!("Not part of the assignment"),
        }

    }

}

pub mod memory {
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct Memory {}
}

pub mod encoding {
    use super::*;
    use operands::{Register, Scale};


    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct Prefix(pub u8);

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct OpCode(pub &'static [u8]);
    impl OpCode {
        pub fn len(&self) -> usize { self.0.len() }
        pub fn bytes(&self) -> Vec<u8> { self.0.to_vec() }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
    pub struct AddressingModes(pub &'static [u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum ModBits {
        /// 0b00
        OO = 0b00,
        /// 0b01
        OI = 0b01,
        /// 0b10
        IO = 0b10,
        /// 0b11
        II = 0b11,
    }
    impl TryFrom<u8> for ModBits {
        type Error = DecodeError;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                0b00 => Ok(ModBits::OO),
                0b01 => Ok(ModBits::OI),
                0b10 => Ok(ModBits::IO),
                0b11 => Ok(ModBits::II),
                _ => Err(DecodeError::DecodeFailure)
            }
        }
    }
    impl From<ModBits> for u8 {
        fn from(value: ModBits) -> Self {
            value as u8
        }
    }
    impl Default for ModBits {
        /// Returns the variant with the value equivalent to 0.
        fn default() -> Self {
            ModBits::OO
        }
    }



    #[allow(unused)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct ModRM (
        /// MOD
        /// Only low two bits are valid
        pub ModBits,
        /// REG
        /// Only low three bits are valid
        pub Register,
        // RM
        // Only low three bits are valid
        pub Register
    ); 
    impl ModRM {

        pub fn precedes_sib_byte(&self) -> bool {
            self.2 == Register::ESP && self.0 != ModBits::II
        }

        pub fn uses_displacement(&self) -> bool {
            (self.0 == ModBits::OO && self.2 == Register::EBP) || 
            self.0 == ModBits::OI ||
            self.0 == ModBits::IO
        }

        /// Returns the different parts of the [ModRM] bytes: (MOD, REG, RM)
        pub fn split(&self) -> (ModBits, Register, Register) { (self.0.clone(), self.1.clone(), self.2.clone()) }

        pub fn as_byte(&self) -> u8 { let byte: u8 = self.into(); byte }

        /// Uses the [ModRM] byte to estimate the number of bytes that _remain_ (after
        /// this [ModRM] byte) in the instruction that this byte may reside in and encode. This
        /// information is derived from Table 2-2 of the Intel Intel64 and IA-32 Arch Manual
        ///
        /// This function does not include lengths that require [Sib] information. If such a case
        /// is encountered then `Self.1` will be false. If `Self.1` is `true`, then returned
        /// remaining bytes is accurate.
        ///
        /// Example: a byte value of `0xF1`, then this function would return `(0, true)` which does 
        /// not include the [ModRM] byte and there are no other bytes in the instruction decode
        pub fn bytes_remaining(&self, sib: Option<Sib>) -> Result<usize, DecodeError> {
            let sib_byte = if sib.is_some() { 1 } else { 0 };
            
            let displacement = match self.0 {
                ModBits::OO => {
                    if self.2 == Register::EBP { 
                        if sib.is_none() { 4 /*disp32*/ } 
                        else {
                            error!("Encountered SIB byte where none was expected");
                            return Err(DecodeError::InvalidSib);
                        }
                    } else
                    if self.2 == Register::ESP { 
                        if sib.is_some() { 
                            let sib = sib.expect("Is some due to conditional");
                            if sib.2 == Register::EBP { 4 /*disp32*/ }
                            else { 0 /*no displacement*/ }
                        }
                        else { 
                            error!("Expected SIB byte and found None");
                            return Err(DecodeError::InvalidSib);
                        }
                    }
                    else { 0 }
                },
                ModBits::OI => { 
                    if self.2 == Register::ESP { 1 /* SIB w/ disp8*/ } 
                    else { 1 }
                },
                ModBits::IO => {
                    if self.2 == Register::ESP { 4 /*SIB w/ disp32*/} 
                    else { 4 }
                },
                ModBits::II => { 0 },
            };

            Ok(sib_byte + displacement)
        }

        #[allow(unused)]
        fn syntax(&self) -> Result<String,DecodeError>  {
            match self.0 {
                ModBits::OO => {
                    Ok("todo".to_string())
                },
                ModBits::OI => { // [r/m + byte]
                    Ok("todo".to_string())
                },
                ModBits::IO => { // [r/m + dword] 
                    Ok("todo".to_string())
                },
                ModBits::II => { // r/m
                    Ok("todo".to_string())
                }
                _ => Err(DecodeError::InvalidAddressingMode),
            }
        }
    }
    impl TryFrom<u8> for ModRM {
        type Error = DecodeError;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            let modbits = ModBits::try_from((value & 0b11000000) >> 6)?;
            let regbits = Register::try_from((value & 0b00111000) >> 3)?;
            let rmbits = Register::try_from((value & 0b00000111) >> 0)?;
            Ok(Self (modbits, regbits, rmbits))
        }
    }
    impl From<&ModRM> for u8 {
        fn from(value: &ModRM) -> Self {
            let modbits = value.0.clone() as u8;
            let regbits = value.1.clone() as u8;
            let rmbits  = value.2.clone() as u8;

            (modbits << 6) |
            (regbits << 3) |
            (rmbits  << 0)
        }
    }

    #[allow(unused)]
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct Sib (
        /// Scale
        // Only low two bits are valid
        Scale,
        /// Index
        // Only low three bits are valid
        Register,
        /// Base
        // Only low three bits are valid
        Register
    ); 
    impl Sib {
        pub fn scale(&self) -> Scale { self.0.clone() }
        pub fn index(&self) -> Register { self.1.clone() }
        pub fn base(&self) -> Register { self.2.clone() }

        #[allow(unused)]
        pub fn bytes_remaining(&self, modrm: ModRM) -> usize {
            if self.1 == Register::ESP { 0 }
            else { 4 }
        }

        pub fn sib(bytes: &[u8], idx: usize) -> Result<Sib, DecodeError> {
            let Some(sib) = bytes.get(idx) 
                else { 
                    error!("Unable to create Sib. Bytes length incorrect");
                    return Err(DecodeError::InvalidLength);
                };
            Sib::try_from(*sib)
        }

    }
    impl TryFrom<u8> for Sib {
        type Error = DecodeError;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            let scale = Scale::try_from(   (value & 0b11000000) >> 6)?;
            let index = Register::try_from((value & 0b00111000) >> 3)?;
            let base  = Register::try_from((value & 0b00000111) >> 0)?;

            // According to Table 2-3, there is no valid sib byte with an
            // Index of 0b100 (ESP register)
            //if index == Register::ESP {
                //error!("Rejecting potential SIB byte");
                //return Err(DecodeError::InvalidSib);
            //}

            Ok(Self (scale, index, base))
        }
    }

    pub mod operands {
        use super::*;

        /// Structure for use in [Operand] for capturing the structure of the operand so it can be
        /// transformed into a string for printing and displaying.
        #[derive(Clone, Debug, PartialEq)]
        pub enum EffectiveAddress {
             /// reg
            Register { reg: Register },

            /// [index*scale + disp32]
            IndexDisp { index: Register, scale: Scale, displacement: Displacement},

            /// [disp32] 
            Displacement { displacement: Displacement },

            /// [base + disp?]
            BaseDisp { base: Register, displacement: Displacement },

            /// [index*scale + base + disp?]
            IndexBaseDisp { index: Register, scale: Scale, base: Register, displacement: Displacement },
        }
        #[allow(unused)]
        impl EffectiveAddress {

            pub fn len(&self) -> usize {
                match self {
                    EffectiveAddress::Register { reg } => 0,
                    EffectiveAddress::IndexDisp { index, scale, displacement } => displacement.len(),
                    EffectiveAddress::Displacement { displacement } => displacement.len(),
                    EffectiveAddress::BaseDisp { base, displacement } => displacement.len(),
                    EffectiveAddress::IndexBaseDisp { index, scale, base, displacement } => displacement.len(),
                }
            }
            
            pub fn from(modrm: ModRM, sib: Option<Sib>, displacement: Displacement) -> Result<EffectiveAddress, DecodeError> {
                debug!("\nMODRM: {modrm:?}\nSIB: {:?}\nDisplacement: {displacement:?}", sib.clone());
                let mod_bits = modrm.0;
                let reg_bits = modrm.1;
                let rm_bits = modrm.2;

                // case:  disp32
                if mod_bits == ModBits::OO && rm_bits == Register::EBP {
                    if matches!(displacement, Displacement::Abs32(_)) {
                        EffectiveAddress::displacement(displacement.get_inner());
                    }
                    else {
                        error!("Expected Displacement::Abs32 and found other"); 
                        return Err(DecodeError::InvalidDisplacementByteWidth);
                    }
                }

                // Cases: 
                // - [--][--]
                // - [--][--] + disp8
                // - [--][--] + disp32
                if modrm.precedes_sib_byte() {
                    let Some(sib) = sib else {
                        error!("Panic! MODRM indicates a SIB Byte but none found!");
                        return Err(DecodeError::InvalidSib);
                    };

                    let scale = sib.0;
                    let index = sib.1;
                    let base = sib.2;

                    let use_base_register = !(base == Register::EBP && mod_bits == ModBits::OO);
                    let displacement_used = !(scale == Scale::One && !use_base_register) &&
                        modrm.uses_displacement();

                    debug!("Use Base Register: {use_base_register}");
                    debug!("Displacement Used: {displacement_used}");



                    if displacement_used && displacement == Displacement::None {
                        error!("Expected Displacement but found DisplacementNone");
                        return Err(DecodeError::InvalidDisplacementByteWidth);
                    }


                    //if !displacement_used && displacement != Displacement::None {
                        //error!("Expected no Displacement but found Displacement");
                        //return Err(DecodeError::InvalidDisplacementByteWidth);
                    //}

                    let ea = match use_base_register {
                        true => { // Form: [base + disp] 
                            if index == Register::ESP {
                                EffectiveAddress::BaseDisp {
                                    base,
                                    displacement,
                                }
                            }
                            else { // Form: [index*scale + base + disp] 
                                EffectiveAddress::IndexBaseDisp {
                                    index,
                                    scale,
                                    base,
                                    displacement,
                                }
                            }
                        },
                        false => { // Form: [index*scale + disp]
                            EffectiveAddress::IndexDisp {
                                index,
                                scale,
                                displacement
                            }
                        }
                    }; 
                    return Ok(ea);
                }


                // End special cases...
                // Remainder of cases:
                // - (0b00) [ REG ]
                // - (0b01) [ REG ] + disp8
                // - (0b10) [ REG ] + disp32
                // - (0b11) REG
                let effective_address = match mod_bits {
                    ModBits::OO => {
                        EffectiveAddress::BaseDisp {
                            base: rm_bits,
                            displacement: Displacement::None,
                        }
                    },
                    ModBits::OI | ModBits::IO => {
                        EffectiveAddress::BaseDisp {
                            base: rm_bits,
                            displacement,
                        }
                    },
                    ModBits::II => {
                        EffectiveAddress::Register {
                            reg: rm_bits,
                        }
                    },
                };
                
                Ok(effective_address)
            }

             /// reg
            pub fn register(reg: Register) -> EffectiveAddress { 
                EffectiveAddress::Register { reg }
            }

             /// [disp]
            pub fn displacement(displacement: u32) -> EffectiveAddress { 
                let displacement = operands::Displacement::Abs32(displacement);
                EffectiveAddress::Displacement{ displacement }
            }

            /// [base]
            pub fn base(base: Register) -> EffectiveAddress {
                EffectiveAddress::BaseDisp { base, displacement: Displacement::None }
            }

            /// [base + disp8]
            pub fn base_d8(base: Register, displacement: u8) -> EffectiveAddress {
                EffectiveAddress::BaseDisp { base, displacement: Displacement::Abs8(displacement) }
            }

            /// [base + disp32]
            pub fn base_d32(base: Register, displacement: u32) -> EffectiveAddress {
                EffectiveAddress::BaseDisp { base, displacement: Displacement::Abs32(displacement) }
            }

            /// [index*scale + base]
            pub fn index_base(index: Register, scale:Scale, base: Register) -> EffectiveAddress {
                EffectiveAddress::IndexBaseDisp{ index, scale, base, displacement: Displacement::None}
            }

            /// [index*scale + base + disp8]
            pub fn index_base_d8(index: Register, scale:Scale, base: Register, displacement: u8) -> EffectiveAddress {
                EffectiveAddress::IndexBaseDisp { index, scale, base, displacement: Displacement::Abs8(displacement) }
            }

            /// [index*scale + base + disp32]
            pub fn index_base_d32(index: Register, scale:Scale, base: Register, displacement: u32) -> EffectiveAddress {
                EffectiveAddress::IndexBaseDisp { index, scale, base, displacement: Displacement::Abs32(displacement) }
            }

            /// [index*scale + disp32]
            pub fn index_d32(index: Register, scale:Scale, displacement: u32) -> EffectiveAddress {
                let displacement = Displacement::Abs32(displacement);
                EffectiveAddress::IndexDisp { index, scale, displacement }
            }
        }
        impl Display for EffectiveAddress {
            /// The different ways in which an [EffectiveAddress] can be rendered to [String] are
            /// dependent on the `address_mode` and which optional elements exist.
            ///
            /// Possibilities are:
            /// - "[ `Register` ]" 
            /// - "[ `Sib` ]"
            /// - "[ `Diplacement::Abs32' ]"
            /// - "[ `Register` + `Diplacement::Abs8` ]"
            /// - "[ `Register` + `Diplacement::Abs32 ]"
            /// - "[ `Sib` + `Diplacement::Abs32 ]"
            /// - "`Register`" 
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use EffectiveAddress::*;
                match self {
                    Register { reg } => { write!(f, "{reg}") 
                    },
                    Displacement { displacement } => {
                        write!(f, "[ {displacement} ]")
                    },
                    IndexDisp { index, scale, displacement } => {
                        match scale {
                            Scale::One => {
                                write!(f, "[ {index}{} ]", displacement.format_optional_operand())
                            },
                            _=> {
                                write!(f, "[ {index} * {scale}{} ]", displacement.format_optional_operand())
                            }
                        }
                    }
                    BaseDisp { base, displacement} => { write!(f, "[ {base}{} ]", displacement.format_optional_operand())}
                    IndexBaseDisp { index, scale, base, displacement } => {
                        write!(f, "[ {index}{} + {base}{} ]",
                            scale.format_optional_operand(),
                            displacement.format_optional_operand())
                    }
                }
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub enum Scale { 
            One = 0, Two = 1, 
            Four = 2, Eight = 3
        }
        impl Scale {
            pub fn format_optional_operand(&self) -> String {
                if *self == Scale::One { format!("") }
                else { format!(" * {self}") }
            }

        }
        impl TryFrom<u8> for Scale {
            type Error = DecodeError;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    0 => Ok(Scale::One),
                    1 => Ok(Scale::Two),
                    2 => Ok(Scale::Four),
                    3 => Ok(Scale::Eight),
                    _ => {
                        error!("Rejecting u8 --> Scale transform");
                        Err(DecodeError::InvalidSib)
                    },
                }
            }
        }
        impl Display for Scale {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let scale = match self { 
                    Scale::One => "1", Scale::Two => "2", 
                    Scale::Four => "4", Scale::Eight => "8" 
                };
                write!(f, "{scale}")
            }
        }

        #[allow(unused)]
        #[derive(Clone, Debug, PartialEq)]
        pub enum Operand {
            Register(Register),
            Immediate(Immediate),
            Displacement(Displacement),
            EffectiveAddress(EffectiveAddress),
            Label(Offset),
        }
        impl Operand {
            #[allow(unused)]
            pub fn displacement(&self) -> Option<Displacement> {
                match self {
                    Operand::Displacement(displacement) => Some(displacement.clone()),
                    _ => None,
                }
            }

            pub fn len(&self) -> usize {
                match self {
                    Operand::Register(_register) => 0,
                    Operand::Immediate(immediate) => immediate.len(),
                    Operand::Displacement(displacement) => displacement.len(),
                    Operand::EffectiveAddress(effective_address) => effective_address.len(),
                    Operand::Label(_) => 0,
                }
            }
        }
        impl Display for Operand {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let out = match self {
                    Operand::Register(register)         => register.to_string(),
                    Operand::Immediate(immediate)       => immediate.to_string(),
                    Operand::Displacement(displacement) => displacement.to_string(),
                    Operand::Label(offset)              => offset.to_string(),
                    Operand::EffectiveAddress(ea) => { ea.to_string()},
                };
                write!(f, "{out}")
            }
        }

        #[derive(Clone, Debug, PartialEq, Default)]
        pub struct Offset(pub u32);
        #[allow(unused)]
        impl Offset {
            pub fn increment(&mut self, bytes: u32) { self.0 += bytes; }
            pub fn to_pointer(&self) -> usize { self.0 as usize }
        }
        impl From<Displacement> for Offset {
            fn from(value: Displacement) -> Self {
                let offset = value.get_inner();
                Offset(offset)
            }
        }
        impl Display for Offset {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "offset_{:08X}h", self.0)
            }
        }

        #[allow(unused)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum Register {
            // Defined by MODRM byte
            EAX = 0, ECX = 1, EDX = 2, EBX = 3,
            ESP = 4, EBP = 5, ESI = 6, EDI = 7,

            // Implied by instruction extension
            AH, AL, AX, RAX,
            BH, BL, BX, RBX,
            CH, CL, CX, RCX,
            DH, DL, DX, RDX,
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
                    _ => {
                        error!("Failed u8 to Register transform");
                        Err(DecodeError::InvalidRegister)
                    },
                }
            }
        }
        impl Default for Register {
            /// Returns the variant with the value equivalent to 0.
            fn default() -> Self {
                Register::EAX
            }
        }
        impl Display for Register {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let reg = match self {
                    Register::AL  => "al",
                    Register::AH  => "ah",
                    Register::AX  => "ax",
                    Register::EAX => "eax",
                    Register::RAX => "rax",

                    Register::BL  => "bl",
                    Register::BH  => "bh",
                    Register::BX  => "bx",
                    Register::EBX => "ebx",
                    Register::RBX => "rbx",

                    Register::CL  => "cl",
                    Register::CH  => "ch",
                    Register::CX  => "cx",
                    Register::ECX => "ecx",
                    Register::RCX => "rcx",

                    Register::DL  => "dl",
                    Register::DH  => "dh",
                    Register::DX  => "dx",
                    Register::EDX => "edx",
                    Register::RDX => "rdx",

                    Register::ESP => "esp",
                    Register::EBP => "ebp",
                    Register::ESI => "esi",
                    Register::EDI => "edi",
                };
                write!(f, "{reg}")
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        #[allow(unused)]
        pub enum Displacement {
            None,
            Rel8(u8), Rel16(u16), Rel32(u32),
            Abs8(u8), Abs16(u16), Abs32(u32)
        }
        impl Displacement {

            /// The number of bytes contained in the Displacement
            /// as seen on disk or in a file
            #[allow(unused)]
            pub fn len(&self) -> usize {
                use Displacement::*;
                match self {
                    None => 0,
                    Rel8(_)  | Abs8(_)  => 1,
                    Rel16(_) | Abs16(_) => 2,
                    Rel32(_) | Abs32(_) => 4,
                }
            }

            pub fn format_optional_operand(&self) -> String {
                if *self == Displacement::None || self.get_inner() == 0 { format!("") } 
                else { format!(" + {self}") }
            }

            pub fn disp8(bytes: &[u8], base: usize) -> Result<Displacement, DecodeError> {
                let Some(displacement) = bytes.get(base..base + 1) 
                    else { 
                        error!("Byte length error when creating Displacement8");
                        return Err(DecodeError::InvalidLength) 
                    };
                let displacement = Displacement::try_from(displacement)?;
                Ok(displacement)
            }

            pub fn disp32(bytes: &[u8], base: usize) -> Result<Displacement, DecodeError> {
                let Some(displacement) = bytes.get(base..base + 4) 
                    else { 
                        error!("Byte length error when creating Displacement32");
                        return Err(DecodeError::InvalidLength)
                    };
                let displacement = Displacement::try_from(displacement)?;
                Ok(displacement)
            }

            /// Returns a u32 of the inner integer. Note this may be upcast 
            /// to [u32].
            pub fn get_inner(&self) -> u32 {
                use Displacement::*;
                match self {
                    None     => 0,
                    Rel8(d)  => *d as u32,
                    Rel16(d) => *d as u32,
                    Rel32(d) => *d,
                    Abs8(d)  => *d as u32,
                    Abs16(d) => *d as u32,
                    Abs32(d) => *d,
                }
            }

            pub fn abs_to_rel(&self, address: Offset, length: usize) -> Result<Displacement, DecodeError> {
                use Displacement::*;

                let base = address.0 + length as u32; //+ operand.len() as u32;

                match self {
                    None     =>  Err(DecodeError::InvalidDisplacementByteWidth),
                    Rel8(d)  =>  Ok(Rel8( *d)),
                    Rel16(d) =>  Ok(Rel16(*d)),
                    Rel32(d) =>  Ok(Rel32(*d)),
                    Abs8(d) => {
                        let base = base as u8 + 1 /*byte*/;
                        let target = d + base;
                        Ok(Rel8(target))
                    },
                    Abs16(d) => {
                        let base = base as u16 + 2 /*bytes*/;
                        let target = d + base;
                        Ok(Rel16(target))
                    },
                    Abs32(d) => {
                        let base = base as u32 + 4 /*bytes*/;
                        let target = d + base;
                        Ok(Rel32(target))
                    }
                }
            }


            /// Attempts to convert a byte range into a relative displacement.
            ///
            /// - *bytes*: a byte list
            /// - *base*: the index in the byte list at which the displacement byte(s) start.
            /// - *width*: byte width of the displacement.
            /// - *
            /// This is.... not the best implementation.
            pub fn from_relative(bytes: &[u8], 
                                 location: Offset, 
                                 opcode_length: usize,
                                 width: usize)
                -> Result<Displacement,DecodeError>
            {
                //let base = location.0 as usize;
                let base = 0;
                let byte_range = base+opcode_length..base+opcode_length+width;
                debug!("Attempting to grab range {:?} from bytes {:?}", byte_range, bytes); 
                let Some(dbytes) = bytes.get(byte_range)
                    else {
                        error!("Index out of bounds when attempting to get bytes for Relative Displacement.");
                        return Err(DecodeError::InvalidModRM);
                    };

                debug!("instruction base: {base}");
                debug!("displacement bytes: {dbytes:X?}");

                let displacement = match dbytes.len() {
                    1 => {
                        let byte = <[u8;1]>::try_from(dbytes).expect("Displacement length calculation should be correct");
                        Displacement::from_byte_relative(location,opcode_length, &byte)
                    },
                    2 => {
                        let bytes = <[u8;2]>::try_from(dbytes).expect("Displacement length calculation should be correct");
                        Displacement::from_word_relative(location, opcode_length, &bytes)
                    }, 
                    4 => {
                        let bytes = <[u8;4]>::try_from(dbytes).expect("Displacement length calculation should be correct");
                        //debug!("Operands: {}", bytes.iter().map(|b| format!("{b:X}")).collect::<Vec<String>>().join(" "));
                        Displacement::from_double_relative(location, opcode_length, &bytes)
                    },
                    _ => return Err(DecodeError::InvalidDisplacementByteWidth),
                };

                Ok(displacement)
            }

            pub fn from_byte_relative(address: Offset, opcode_length: usize, operand: &[u8;1]) -> Displacement {
                let base = address.0 + opcode_length as u32 + operand.len() as u32;
                let displacement = operand[0];
                if ((displacement & 0x80) >> 7) == 1 {
                    let target = ((displacement as i8) as i32) + base as i32;
                    debug!("Target = {displacement} + {base}");
                    Displacement::Rel8(target as u8)
                } 
                else {
                    let target = displacement as u32 + base as u32;
                    debug!("Target = {displacement} + {base}");
                    Displacement::Rel8(target as u8)
                }
            }

            pub fn from_word_relative(address: Offset, opcode_length: usize, operand: &[u8;2]) -> Displacement {
                let base = address.0 as usize + opcode_length + operand.len();

                //let displacement = word_to_double_with_sign_extend(*operand);
                let displacement = u16::from_be_bytes(*operand);
                let target = displacement + base as u16; 
                Displacement::Rel16(target)
            }

            pub fn from_double_relative(address: Offset, opcode_length: usize, operand: &[u8;4]) -> Displacement {
                let base = address.0 + opcode_length as u32 + operand.len() as u32;

                let displacement = u32::from_le_bytes(*operand);
                debug!("displacement: {displacement:X}");
                let target = displacement + base;
                Displacement::Rel32(target)
            }
        }
        impl Default for Displacement {
            fn default() -> Self {
                Displacement::None
            }
        }
        impl From<Displacement> for u32 {
            fn from(value: Displacement) -> Self { value.get_inner() }
        }
        impl From<&Displacement> for u32 {
            fn from(value: &Displacement) -> Self { value.get_inner() }
        }
        impl TryFrom<&[u8]> for Displacement {
            type Error = DecodeError;

            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                let displacement = match value.len() {
                    1 => { Displacement::Abs8(value[0]) },
                    2 => {
                        let Ok(displacement) = <[u8;2]>::try_from(value) 
                        else { return Err(DecodeError::DecodeFailure); };

                        // Is this the right thing? Endianess hurts my brain...
                        let displacement = u16::from_le_bytes(displacement);
                        Displacement::Abs16(displacement)
                    },
                    4 => {
                        let Ok(displacement) = <[u8;4]>::try_from(value) 
                        else { return Err(DecodeError::DecodeFailure); };

                        // Is this the right thing? Endianess hurts my brain...
                        let displacement = u32::from_le_bytes(displacement);
                        Displacement::Abs32(displacement)
                    },
                    _ => { return Err(DecodeError::InvalidLength); }
                };
                Ok(displacement)
            }
        }
        impl Display for Displacement {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let string = match self {
                    // This case may be incorrect
                    Displacement::Rel8(d)  =>  {let offset = Offset(*d as u32); offset.to_string()}, 
                    Displacement::Rel16(d) =>  {let offset = Offset(*d as u32); offset.to_string()},
                    Displacement::Rel32(d) =>  {let offset = Offset(*d);        offset.to_string()},

                    // This will not work long term. 
                    Displacement::Abs8(d)  => {
                        if ((d & 0b1000_0000) >> 7) == 1 {
                            format!("0x{:08X}", (*d as i8) as i32)
                        } else {
                            format!("0x{d:08X}")
                        }
                    },
                    Displacement::Abs16(d) => {
                        if ((d & 0x8000) >> 15) == 1 {
                            format!("0x{:08X}", (*d as i16) as i32)
                        } else {
                            format!("0x{d:08X}")
                        }
                    },
                    Displacement::Abs32(d) => {
                        if ((d & 0x80000000) >> 31) == 1 {
                            format!("0x{:08X}", (*d as i32))
                        } else {
                            format!("0x{d:08X}")
                        }
                    },
                    Displacement::None => "".into(),
                };
                write!(f, "{string}")
            }
        }

        #[allow(unused)]
        fn byte_to_double_with_sign_extend(bytes: [u8;1]) -> [u8;4] {
            if bytes[0].leading_zeros() == 0 {
                [0xFF, 0xFF, 0xFF, bytes[0]]
            } else {
                [0x00, 0x00, 0x00, bytes[0]]
            }
        }

        #[allow(unused)]
        fn byte_to_word_with_sign_extend(bytes: [u8;1]) -> [u8;2] {
            if bytes[0].leading_zeros() == 0 {
                [0xFF, bytes[0]]
            } else {
                [0x00, bytes[0]]
            }
        }

        #[allow(unused)]
        fn word_to_double_with_sign_extend(bytes: [u8;2]) -> [u8;4] {
            if bytes[0].leading_zeros() == 0 {
                [0xFF, 0xFF, bytes[0], bytes[1]]
            } else {
                [0x00, 0x00, bytes[0], bytes[1]]
            }
        }

        #[test]
        fn rel8_calculation() {
            let expected_a = 0x80;

            let address = Offset(0x10);
            let opcode_length = 1;
            let operand: &[u8] = &[0x6E];
            let base = address.0 + opcode_length + operand.len() as u32;
            debug!("{base:x}");

            let displacement = <[u8;1]>::try_from(operand).unwrap();
            let displacement = byte_to_double_with_sign_extend(displacement);
            let displacement = u32::from_be_bytes(displacement);
            debug!("{displacement:x}");
            let target = displacement + base;
            debug!("{target:x} ?= {expected_a:x}");
            assert_eq!(target, expected_a);
        }

        #[test]
        fn rel32_calculation() {
            let expected_a = 0xAABBCCDD;

            let address = Offset(0x1000);
            let opcode_length = 1;
            let operand: &[u8] = &[0xAA, 0xBB, 0xBC, 0xD8];
            let base = address.0 + opcode_length + operand.len() as u32;
            debug!("{base:x}");

            let displacement = <[u8;4]>::try_from(operand).unwrap();
            let displacement = u32::from_be_bytes(displacement);
            debug!("{displacement:x}");
            let target = displacement + base;
            debug!("{target:x} ?= {expected_a:x}");
            assert_eq!(target, expected_a);
        }

        #[allow(unused)]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub enum Immediate {
            Imm8(Vec<u8>),
            Imm16(Vec<u8>),
            Imm32(Vec<u8>),
            Imm64(Vec<u8>),
        } 
        impl Immediate {
            pub fn raw_bytes(&self) -> Vec<u8> {
                let bytes = match self {
                    Immediate::Imm8(vec) =>  vec,
                    Immediate::Imm16(vec) => vec,
                    Immediate::Imm32(vec) => vec,
                    Immediate::Imm64(vec) => vec,
                };
                bytes.clone()
            }

            pub fn len(&self) -> usize { 
                match self {
                    Immediate::Imm8(vec) => vec.len(),
                    Immediate::Imm16(vec) => vec.len(),
                    Immediate::Imm32(vec) => vec.len(),
                    Immediate::Imm64(vec) => vec.len(),
                }

            }
        }
        impl TryFrom<&[u8]> for Immediate {
            type Error = DecodeError;

            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                match value.len() {
                    1 => Ok(Immediate::Imm8(value.to_vec())),
                    2 => Ok(Immediate::Imm16(value.to_vec())),
                    4 => Ok(Immediate::Imm32(value.to_vec())),
                    8 => Ok(Immediate::Imm64(value.to_vec())),
                    _ => Err(DecodeError::InvalidImmediateSize(value.len()))

                }
            }
        }
        impl Display for Immediate {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let bytes = self.raw_bytes()
                    .iter()
                    .rev()
                    .map(|b| format!("{b:02X}"))
                    .collect::<Vec<String>>()
                    .join("");
                write!(f, "0x{bytes}")
            }
        }
    }

    pub mod extensions {
        use super::*;

        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub struct ExtSet(pub &'static [&'static str]);
        impl ExtSet {

            #[allow(unused)]
            pub fn contains(&self, rhs: Extension) -> bool {
                let result: Vec<bool> = self.0
                    .iter()
                    .filter_map(|ext| {
                        let lhs = Extension::try_from(*ext);
                        if lhs.is_err() { None }
                        else {
                            let lhs = lhs.expect("Should be Ok from conditional");
                            if lhs == rhs { Some(true) }
                            else { None }
                        }
                    }).collect();
                result.contains(&true)
            }

            /// This function will yield the _first_ sdigit (/digit) extension defined by the
            /// decoding rule. 
            ///
            /// Assumption is that encoding rules make use of one and only one sdigit 
            /// extension.
            pub fn get_sdigit(&self) -> Option<Extension> {
                self.0.iter()
                    .find_map(|ext| {
                        let Ok(ext) = Extension::try_from(*ext) 
                            else { return None };
                        if ext.is_sdigit() { Some(ext) }
                        else { None }
                    })
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub enum Extension {
            RW, RD,
            IB, IW, ID,
            CB, CW, CD,
            SR,
            S0, S1, S2, S3, S4, S5, S6, S7,
            Rel8, Rel16, Rel32,

        }
        impl Extension {
            pub fn is_sdigit(&self) -> bool {
                match self {
                    Extension::S0 |
                    Extension::S1 |
                    Extension::S2 |
                    Extension::S3 |
                    Extension::S4 |
                    Extension::S5 |
                    Extension::S6 |
                    Extension::S7 => true,
                    _ => return false,
                }
            }

            pub fn valid_sdigit(&self, value: u8) -> bool {
                let sdigit = match self {
                    Extension::S0 => 0,
                    Extension::S1 => 1,
                    Extension::S2 => 2,
                    Extension::S3 => 3,
                    Extension::S4 => 4,
                    Extension::S5 => 5,
                    Extension::S6 => 6,
                    Extension::S7 => 7,
                    _ => return false,
                }; 
                sdigit == value
            }

            pub fn operand_length(&self) -> Option<usize> {
                match self {
                    Extension::IB => Some(1),
                    Extension::IW => Some(2),
                    Extension::ID => Some(4),
                    Extension::RD => Some(0),
                    Extension::RW => Some(0),
                    Extension::Rel8  => Some(1),
                    Extension::Rel16 => Some(2),
                    Extension::Rel32 => Some(4),
                    _ => None, // Do the others encode operand length?
                }
            }
        }
        impl TryFrom<&'static str> for Extension {
            type Error = DecodeError;

            fn try_from(value: &'static str) -> Result<Self, Self::Error> {
                match value {
                    "+rw"   => Ok(Extension::RW),
                    "+rd"   => Ok(Extension::RD),
                    "ib"    => Ok(Extension::IB),
                    "iw"    => Ok(Extension::IW),
                    "id"    => Ok(Extension::ID),
                    "cb"    => Ok(Extension::CB),
                    "cw"    => Ok(Extension::CW),
                    "cd"    => Ok(Extension::CD),
                    "/r"    => Ok(Extension::SR),
                    "/0"    => Ok(Extension::S0),
                    "/1"    => Ok(Extension::S1),
                    "/2"    => Ok(Extension::S2),
                    "/3"    => Ok(Extension::S3),
                    "/4"    => Ok(Extension::S4),
                    "/5"    => Ok(Extension::S5),
                    "/6"    => Ok(Extension::S6),
                    "/7"    => Ok(Extension::S7),
                    "rel8"  => Ok(Extension::Rel8),
                    "rel16" => Ok(Extension::Rel16),
                    "rel32" => Ok(Extension::Rel32),
                    _ => Err(DecodeError::InvalidOpCodeExtension)
                }
            }
        }
    }
}
