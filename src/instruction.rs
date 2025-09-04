
#[allow(unused)]
#[derive(Clone, Debug)]
pub enum DecodeError {
    InvalidAddressingMode
}

/// ADD-Add 
/// [Opcode] [Instruction] [`Op/En`](OperandEncoding) 64-bit Mode Compat/Leg Mode Description
///
/// 0x81: [ # OPCODE
///         None,          # Option<Mnemonic>
///         True,          # ModRMByte
///         'mi',          # OpEn
///         {              # OpcodeExtension Dictionary
///             0: 'add', 1: 'or', 2: 'adc', 3: 'sbb', 
///             4: 'and', 5: 'sub', 6: 'xor', 7: 'cmp'
///          }
/// ]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instruction {
    opcode: OpCode,
    modrm: ModRM,
    sib: Sib,
    displacement: Displacement,
}
impl TryFrom<String> for Instruction {
    type Error = DecodeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let ins = Instruction {
            opcode: OpCode::A,
            modrm: ModRM::default(),
            sib: Sib::default(),
            displacement: todo!(),
        };
        Ok(ins)
    }
}
impl TryFrom<&[u8]> for Instruction {
    type Error = DecodeError;

    fn try_from(_value: &[u8]) -> Result<Self, Self::Error> {
        let ins = Instruction {
            opcode: OpCode::A,
            modrm: ModRM::default(),
            sib: Sib::default(),
            displacement: todo!(),
        };
        Ok(ins)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum OpCode {
    A
}

#[allow(unused)]
#[derive(Clone, Debug)]
enum OperandEncoding {
    RM,
    MR,
    MI,
    I
}

#[allow(unused)]
#[derive(Clone, Debug)]
enum Operand {
    Memory,
    Reg(Registers),
    Imm { width: usize, value: u32 },
    None
}

#[allow(unused)]
#[derive(Clone, Debug)]
enum Registers {
    EAX = 0, ECX = 1, EDX = 2, EBX = 3,
    ESP = 4, EBP = 5, ESI = 6, EDI = 7,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Displacement(u8);

#[allow(unused)]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct ModRM {
    r#mod: u8,
    reg:   u8,
    rm:    u8
} 
impl ModRM {
    fn syntax(&self) -> Result<String,DecodeError>  {
        match self.r#mod {
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
        Self {
            r#mod: (value & 0b11000000) >> 6,
            reg:   (value & 0b00111000) >> 3,
            rm:    (value & 0b00000111) >> 0,
        }
    }
}

#[allow(unused)]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Sib {
    scale: u8,
    index: u8,
    base:  u8
} 
impl From<u8> for Sib {
    fn from(value: u8) -> Self {
        Self {
            scale: (value & 0b11000000) >> 6,
            index:   (value & 0b00111000) >> 3,
            base:    (value & 0b00000111) >> 0,
        }
    }
}
