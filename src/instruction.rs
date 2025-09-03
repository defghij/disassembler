
#[allow(unused)]
#[derive(Clone, Debug)]
pub struct InstructionDecodeError(String);

/// ADD-Add 
/// [Opcode] [Instruction] [`Op/En`](OperandEncoding) 64-bit Mode Compat/Leg Mode Description
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instruction {
    opcode: OpCode,
    instruction: String,
    operand_encoding: OpEncoding,
    description: Option<String>,
}
impl TryFrom<String> for Instruction {
    type Error = InstructionDecodeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let ins = Instruction {
            opcode: OpCode::A,
            instruction: value,
            operand_encoding: OpEncoding::A,
            description: None,
        };
        Ok(ins)
    }
}
impl TryFrom<&[u8]> for Instruction {
    type Error = InstructionDecodeError;

    fn try_from(_value: &[u8]) -> Result<Self, Self::Error> {
        let ins = Instruction {
            opcode: OpCode::A,
            instruction: "none".to_string(),
            operand_encoding: OpEncoding::A,
            description: None,
        };
        Ok(ins)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum OpCode {
    A
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum OpEncoding {
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
    RM,
    REG,
    Imm,
    EAX,
    NONE
}

#[allow(unused)]
#[derive(Clone, Debug)]
enum Registers {
    EAX = 0, ECX = 1, EDX = 2, EBX = 3,
    ESP = 4, EBP = 5, ESI = 6, EDI = 7,
}

#[allow(unused)]
#[derive(Clone, Debug)]
struct ModRM {
    r#mod: u8,
    reg:   u8,
    rm:    u8
}
