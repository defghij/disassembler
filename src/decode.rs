use std::fmt::Display;

use tracing::{debug, error};
use crate::instruction::encoding::{operands::{EffectiveAddress, Offset}, ModBits};
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

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum DecodeError {
    DecodeFailure,
    NoBytesPresent,
    UnknownOpcode,
    InvalidModRM,
    InvalidSib,
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

        let opcode: u8 = op_code.0[0];
        let opcode_length = rule.op_code().len();
        let mut modrm_idx = 0;
        let mut sib_idx   = 0;
        let mut disp_idx  = 0;
        let mut disp_len  = 0;
        let mut imm_idx   = 0;
        let mut instruction_length = opcode_length;

        let modrm = if rule.modrm_required() {
            modrm_idx = instruction_length; // zero indexing
            instruction_length += 1;
            let modrm = rule.modrm_byte(bytes[modrm_idx])?;
            rule.validate_addressing_mode(modrm)?;
            Some(modrm)
        } else { None };

        let sib = if modrm.is_some_and(|m| m.precedes_sib_byte()) {
            sib_idx = instruction_length; // zero indexing
            instruction_length += 1;
            let sib = Sib::sib(bytes, sib_idx)?;
            Some(sib)
        } else { None };

        disp_idx = instruction_length;

        

        //let bytes_remaining = modrm.bytes_remaining(sib);

        let mut instruction = Instruction::new(mnemonic);

        let instruction = match op_encode {
            OpEn::O  => {
                // All Single byte OpEn::O instructions _should have one and only one extension
                // "/rd"
                if extensions.is_some_and(|ext| ext.len() ==1 && ext[0] == Extension::RD) { 
                    let byte = bytes[0];
                    let register = Register::try_from(byte - opcode)?;

                    instruction.add(Operand::Register(register));

                    Bytes::Decoded {
                        bytes: vec![byte],
                        instruction: instruction.clone()
                    }
                }
                else { return Err(DecodeError::DecodeFailure); }
            },
            OpEn::ZO => {
                if extensions.is_none() { 
                    Bytes::Decoded {
                        bytes: vec![bytes[0]],
                        instruction: instruction.clone()
                    }
                }
                else { return Err(DecodeError::InvalidOpCodeExtension); } 
            },
            OpEn::I  => {
                // Validate instruction assumptions.
                if op_code.len() != 1   { 
                    error!("OpCode length incorrect"); 
                    return Err(DecodeError::InvalidOpCodeLength);
                }
                if !extensions.as_ref().is_some_and(|exts| exts.len() == 1) { 
                    error!("Incorrect number of OpCode Extensions");
                    return Err(DecodeError::InvalidOpCodeExtension); 
                }
                let extensions = extensions.expect("Is some due to conditional above");

                if rule.implicit_operand().is_some() {
                    let register = rule.implicit_operand().unwrap();
                    instruction.add(Operand::Register(register));
                }

                imm_idx += instruction_length; // opcode + imm

                let imm = Bytes::decode_immediate(bytes, imm_idx, extensions)?;
                instruction_length += imm.len();
                instruction.add(Operand::Immediate(imm));

                Bytes::Decoded {
                    bytes: bytes[0..instruction_length].to_vec(),
                    instruction: instruction.clone()
                }
            }
            OpEn::D  => {
                let width = rule.len().0 - opcode_length;
                debug!("Displacement Byte Width: {width}");

                let displacement = Displacement::from_relative(bytes, location, opcode_length, width)?;
                instruction_length += displacement.len();

                instruction.add(Operand::Displacement(displacement));

                Bytes::Decoded {
                    bytes: bytes[0..instruction_length].to_vec(),
                    instruction: instruction.clone(),
                }
            }
            OpEn::OI => {
                let Some(extensions) = extensions 
                else { 
                    error!("Expected OpCode extensions. None found");
                    return Err(DecodeError::InvalidOpCodeExtension); 
                };

                if extensions.contains(&Extension::RD) {
                    let register = Register::try_from(bytes[0] - opcode)?;
                    instruction.add(Operand::Register(register));
                }

                imm_idx = instruction_length;

                let imm = Bytes::decode_immediate(bytes, imm_idx, extensions)?;
                instruction_length += imm.len();
                instruction.add(Operand::Immediate(imm));

                Bytes::Decoded {
                    bytes: bytes[..instruction_length].to_vec(),
                    instruction: instruction.clone()
                }
            }
            OpEn::M  => { 
                debug!("OpEn::M");
                let Some(modrm) = modrm else { 
                    error!("This OpEn requires a MODRM byte but found None");
                    return Err(DecodeError::InvalidModRM);
                };
                
                // All declared OpEn::M `DecodeRules` have an extension.
                let Some(extensions) = extensions 
                else { 
                    error!("Expected OpCode extensions. None found");
                    return Err(DecodeError::InvalidOpCodeExtension)
                };

                let operand = Bytes::decode_memory(bytes, disp_idx, modrm, sib)?;
                instruction_length += operand.len();
                instruction.add(operand);

                Bytes::Decoded {
                    bytes: bytes[..instruction_length].to_vec(),
                    instruction: instruction.clone()
                }
            },
            OpEn::M1 => {
                debug!("OpEn::M1");
                let Some(modrm) = modrm else { 
                    error!("This OpEn requires a MODRM byte but found None");
                    return Err(DecodeError::InvalidModRM);
                };
                
                let operand = Bytes::decode_memory(bytes, disp_idx, modrm, sib)?;
                instruction_length += operand.len();
                instruction.add(operand);
                instruction.add(Operand::Immediate(Immediate::Imm8(vec![1])));


                Bytes::Decoded {
                    bytes: bytes[..instruction_length].to_vec(),
                    instruction,
                }
            },
            OpEn::MI => { 
                debug!("OpEn::MI");
                
                // Take care of the M part of MI
                /////////////////////////////////////////

                let Some(modrm) = modrm else { 
                    error!("This OpEn requires a MODRM byte but found None");
                    return Err(DecodeError::InvalidModRM);
                };
                
                let mut displacement = Displacement::None;

                if sib.as_ref().is_some_and(|s| s.base() == Register::EBP && modrm.0 == ModBits::OO ){
                    displacement = Displacement::disp32(bytes, disp_idx)?;
                } else 
                if modrm.uses_displacement() {
                    displacement = match modrm.0 {
                        ModBits::OO | ModBits::IO => Displacement::disp32(bytes, disp_idx)?,
                        ModBits::OI => Displacement::disp8(bytes, disp_idx)?,
                        _ => { Displacement::None }
                    };
                }
                else {
                    displacement = Displacement::None;
                };

                instruction_length += displacement.len();

                let effective_address = EffectiveAddress::from(modrm, sib, displacement)?;

                instruction.add(Operand::EffectiveAddress(effective_address));

                // Take care of the I part of MI
                ///////////////////////////////////////////////
                imm_idx = instruction_length;
                debug!("imm_idx: {imm_idx}");
                
                if !extensions.as_ref().is_some_and(|exts| exts.len() == 2) { 
                    error!("Incorrect number of OpCode Extensions. Expected 2");
                    return Err(DecodeError::InvalidOpCodeExtension); 
                }
                let extensions = extensions.expect("Is some due to conditional above");

                let imm = Bytes::decode_immediate(bytes, imm_idx, extensions)?;
                instruction_length += imm.len();
                instruction.add(Operand::Immediate(imm));

                Bytes::Decoded {
                    bytes: bytes[..instruction_length].to_vec(),
                    instruction,
                }
            },
            OpEn::MR => { 
                let Some(modrm) = modrm else { 
                    error!("This OpEn requires a MODRM byte but found None");
                    return Err(DecodeError::InvalidModRM);
                };
                
                // First instruction is Memory
                let operand = Bytes::decode_memory(bytes, disp_idx, modrm, sib)?;
                instruction_length += operand.len();
                instruction.add(operand);

                // Second operand is Register
                if extensions.is_some_and(|e| e.contains(&Extension::SR)) {
                    instruction.add(Operand::Register(Register::from(modrm.1)));
                } else {
                    error!("Operand Encoding expected an extension `/r` but found none");
                    return Err(DecodeError::InvalidOpCodeExtension);
                }

                Bytes::Decoded {
                    bytes: bytes[..instruction_length].to_vec(),
                    instruction,
                }
            },
            OpEn::RM => { todo!() },
            OpEn::NP => { todo!() },
            OpEn::FD => { todo!() },
            OpEn::TD => { todo!() },
        };

        Ok(instruction)
    }

    pub fn decode_memory(bytes: &[u8], idx: usize, modrm: ModRM, sib: Option<Sib>) -> Result<Operand, DecodeError> {
        debug!("Mod bits: {:?}", modrm.0);
        let operand = match modrm.0 {
            ModBits::OO => {
                match modrm.2 {
                    Register::ESP => { // [--][--]
                        let Some(sib) = sib 
                            else {
                                error!("Required SIB byte not found");
                                return Err(DecodeError::InvalidSib);
                            };
                        debug!("RM bits: {:?}", modrm.2);

                        let eaddr  = if sib.base() == Register::EBP {
                                let displacement = Displacement::disp32(bytes, idx)?;
                                EffectiveAddress::from(modrm, Some(sib), displacement)?
                            } else { EffectiveAddress::from(modrm, Some(sib), Displacement::None)? };
                        Operand::EffectiveAddress(eaddr)

                    },
                    Register::EBP => { 
                        debug!("RM bits: {:?}", modrm.2);

                        let displacement = Displacement::disp32(bytes, idx)?;

                        let displacement = EffectiveAddress::displacement(displacement.into());

                        Operand::EffectiveAddress(displacement)
                    },
                    _     => { 
                        debug!("RM bits: {:?}", modrm.2);
                        let register = EffectiveAddress::base(modrm.2);

                        Operand::EffectiveAddress(register)
                    }

                }
            },
            ModBits::OI => {
                match modrm.2 {
                    Register::ESP => { // [--][--]+disp8
                        debug!("RM bits: {:?}", modrm.2);
                        let displacement = Displacement::disp8(bytes, idx)?;
                        let effective_address = EffectiveAddress::from(modrm, sib, displacement)?;
                        debug!("Effective: {effective_address}");
                        Operand::EffectiveAddress(effective_address)
                    },
                    _ => { 
                        debug!("RM bits: {:?}", modrm.2);
                        let displacement = Displacement::disp8(bytes, idx)?;
                        let d: u32 = displacement.into();

                        let displacement = EffectiveAddress::base_d8(modrm.2, d as u8);

                        Operand::EffectiveAddress(displacement)
                    },
                }

            },
            ModBits::IO => {
                match modrm.2 {
                    Register::ESP => { // [--][--]+disp32
                        debug!("RM bits: {:?}", modrm.2);

                        let displacement = Displacement::disp32(bytes, idx)?;

                        let eaddr = EffectiveAddress::from(modrm, sib, displacement)?;

                        Operand::EffectiveAddress(eaddr)
                    },
                    _ => { 
                        debug!("RM bits: {:?}", modrm.2);

                        let displacement = Displacement::disp32(bytes, idx)?;

                        let eaddr = EffectiveAddress::base_d32(modrm.2, displacement.into());

                        Operand::EffectiveAddress(eaddr)
                    },
                }
            },
            ModBits::II => {
                debug!("Mod bits: {:?}", modrm.0);
                Operand::Register(modrm.2)
            }
        };
        Ok(operand)
    }

    pub fn decode_immediate(bytes: &[u8], idx: usize, extensions: Vec<Extension>) -> Result<Immediate, DecodeError> {
        if extensions.contains(&Extension::IB) { 
            let Some(imm) = bytes.get(idx..idx + 1) 
                else {
                    error!("Attempted to grab more bytes than where handed to function for instruction decode");
                    return Err(DecodeError::InvalidLength);
                };
            Ok(Immediate::Imm8(imm.to_vec()))
        } else 
        if extensions.contains(&Extension::IW) { 
            error!("Extension for Immediate Word is not implemented"); 
            return Err(DecodeError::DecodeFailure);
        } else
        if extensions.contains(&Extension::ID) { 
            let Some(imm) = bytes.get(idx.. idx + 4) 
                else {
                    error!("Attempted to grab more bytes than where handed to function for instruction decode");
                    return Err(DecodeError::InvalidLength);
                };
            Ok(Immediate::Imm32(imm.to_vec()))
        } else {
            error!("Encountered error case when attempting to decode an immediate");
            Err(DecodeError::DecodeFailure)
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

    pub fn validate_addressing_mode(&self, modrm: ModRM) -> Result<(),DecodeError> {
        if !self.5.as_ref().is_some_and(|a| a.0.contains(&modrm.0.into())) { 
            error!("Encountered invalid addressing mode: {:?}", modrm.0);
            return Err(DecodeError::InvalidAddressingMode); 
        } 
        else { Ok(()) }
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
            OpEn::M | OpEn::M1 | OpEn::MI | OpEn::MR | OpEn::RM => {
                let extensions = extensions.as_ref().expect("All Rules with an OpEn::M encoding should require an extension");
                let mut bytes = extensions.iter()
                    .filter(|ext| ext.operand_length().is_some())
                    .fold(0, |acc, ext| acc + ext.operand_length().expect("Should be some due to fiter") );

                if self.modrm_required() { bytes += 1 }
                (op_code.len() + bytes, false)
            },
            OpEn::NP => unimplemented!("`len` not implemented for this Operand Encoding"),
            OpEn::ZO => unimplemented!("`len` not implemented for this Operand Encoding"),
            OpEn::O  => unimplemented!("`len` not implemented for this Operand Encoding"),
            OpEn::FD => unimplemented!("`len` not implemented for this Operand Encoding"),
            OpEn::TD => unimplemented!("`len` not implemented for this Operand Encoding"),
        }
    }

    pub fn modrm_required(&self) -> bool {
        self.4.modrm_required()
    }

    pub fn extensions_required(&self) -> bool {
        self.3.is_some()
    }

    pub fn uses_sdigit(&self) -> bool {
        self.3.clone().is_some_and(|exts|{
            exts.get_sdigit().is_some()
        })
    }

    /// Takes a [u8] and yields a Some([ModRM]) if the byte can be validated as a ModRM byte for the
    /// particular [DecodeRule] that self describes. If is not valid, then a [None] is returned.
    pub fn modrm_byte(&self, byte: u8) -> Result<ModRM, DecodeError> {
        let ext_set      = self.3.clone();
        let addr_mode    = self.5.clone(); 
        let modrm        = ModRM::try_from(byte)?; 
        let (md, rg, rm) = modrm.split();

        // Check that if extension dictates a value in the modrm byte that it is set.
        if ext_set.is_some() {
            let extensions = ext_set.as_ref().expect("Should be some due to conditional");
            if self.uses_sdigit() {
                debug!("Uses sdigit");
                let Some(sdigit) = extensions.get_sdigit() else { 
                    error!("Expected SDigit extension. Found None");
                    return Err(DecodeError::InvalidModRM);
                };

                // Decoding rule and ModRM have incompatible REG bits.
                if !sdigit.valid_sdigit(rg as u8) { 
                    error!("Rule (sdigit) and ModRM (REG bits) conflict");
                    return Err(DecodeError::InvalidModRM) }
            }
        }


        // Check that addressing mode is valid for this rule
        if addr_mode.is_some() {
            let addressing_mode = addr_mode.as_ref().expect("Should be some due to conditional");

            // Decoding rule and ModRM have incompatible MOD bits
            if !addressing_mode.0.contains(&md.into()) {
                error!("Rule (addressing mode) and ModRM (MOD bits) conflict");
                return Err(DecodeError::InvalidModRM) 
            }
        }
        Ok(modrm)
    }

    pub fn implicit_operand(&self) -> Option<Register> {
        let (_, _, op_code, extensions, op_encoding, _) = self.separate();
        if op_encoding == OpEn::I && extensions.is_some() {
            let extensions = extensions.expect("Should be Some by virtue of above conditional");

            match op_code.bytes()[0] {
                // OpCodes where an operands implied by the OpCode
                0x2D | 0x05 | 0x39 => {
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

    pub fn can_make_label(&self) -> bool {
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

