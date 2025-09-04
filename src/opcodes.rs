use phf::{phf_map, Map};

use super::{
    decode::DecodeRule,
    instruction::{
        OpEn,
        encoding::{
            Prefix,
            AddressingModes,
            OpCode,
            extensions::ExtSet
        },
    }
};

/// Byte --> Presumptive Decode Rules mapping
pub type InsMap = Map<u8, DecodeRule>;

/// Complete Opcode and Addressing Mode Requirements
pub static OPCODES: [InsMap; 21] = [
    ADD,  AND, CALL, CLFLUSH, CMP, DEC, IDIV, INC, 
    JMP, JCMP,  LEA,     MOV, NOP, NOT,   OR, POP, 
    PUSH, SUB, TEST,     XOR,  REPNECMPSD,         
];

/// Returns a Vector of possible ways to decode the current and
/// potentially follow on bytes based _only_ on the a single byte.
///
/// *Assumption*:
/// Relies on the assumption that the [`Map<u8,DecodeRule>`]s contained
/// in OPCODES has a one-to-one relationship between key,value pairs.
pub fn presumptive_decode_rules(byte: u8) -> Vec<DecodeRule> {
    //OPCODES.iter()
        //.filter_map(|opcode_class: InsMap|{
            //if opcode_class.contains_key(opcode_class) {
                //opcode_class
            //} else { None }
        //})
    unimplemented!("lol");
}


macro_rules! ins0 {
    ($Mnemonic:literal, $OpCodes:expr, $OpEn:expr) => {
        DecodeRule($Mnemonic, 
                   None, 
                   OpCode(&$OpCodes), 
                   None,  
                   Some($OpEn), 
                   None
        )
    };
}


macro_rules! ins1 {
    ($Mnemonic:literal, $OpCodes:expr, $extensions:expr, $OpEn:expr) => {
        DecodeRule($Mnemonic,
                   None, 
                   OpCode(&$OpCodes), 
                   Some(ExtSet(&$extensions)), 
                   Some($OpEn), 
                   None
        )
    };
}

macro_rules! ins2 {
    ($Mnemonic:literal, $Prefix:expr, $OpCodes:expr, $OpEn:expr) => {
        DecodeRule($Mnemonic, 
                   Some(Prefix($Prefix)), 
                   OpCode(&$OpCodes), 
                   None,  
                   Some($OpEn), 
                   None
        )
    };
}

macro_rules! ins3 {
    ($Mnemonic:literal, $OpCodes:expr, $extensions:expr, $OpEn:expr, $AddrModes:expr) => {
        DecodeRule($Mnemonic, 
                   None, 
                   OpCode(&$OpCodes), 
                   Some(ExtSet(&$extensions)),  
                   Some($OpEn), 
                   Some(AddressingModes(&$AddrModes))
        )
    };
}

//static BYTE_TO_DECODE_RULES: Map<u8, &'static [DecodeRule]> = phf_map! {
    //0x05 => &[ins1!("add",  [0x05], ["id"],       OpEn::I)],
    //0x81 => &[ins3!("add",  [0x81], ["/0", "id"], OpEn::MI, [0b00,0b01,0b10,0b11])],
    //0x01 => &[ins3!("add",  [0x01], ["/r"],       OpEn::MR, [0b00,0b01,0b10,0b11])],
    //0x03 => &[ins3!("add",  [0x03], ["/r"],       OpEn::RM, [0b00,0b01,0b10,0b11])],
    //0x25 => ins1!("and",  [0x25], ["id"],       OpEn::I),
    //0x81 => ins3!("and",  [0x81], ["/4", "id"], OpEn::MI, [0b00,0b01,0b10,0b11]),
    //0x21 => ins3!("and",  [0x21], ["/r"],       OpEn::MR, [0b00,0b01,0b10,0b11]),
    //0x23 => ins3!("and",  [0x23], ["/r"],       OpEn::RM, [0b00,0b01,0b10,0b11]),
    //0xE8 => ins1!("call",  [0xE8], ["id"], OpEn::D), // Is this right?
    //0xFF => ins3!("call",  [0xFF], ["/2"], OpEn::M, [0b00,0b01,0b10,0b11]),
    //0xFF => ins3!("clflush",  [0xAE], ["/7"], OpEn::M, [0b00,0b01,0b10,]), // ???
    //0x3D => ins1!("cmp",  [0x3D], ["/id"],      OpEn::I), 
    //0x81 => ins3!("cmp",  [0x81], ["/7", "id"], OpEn::MI, [0b00,0b01,0b10,0b11]), 
    //0x39 => ins3!("cmp",  [0x39], ["/r"],       OpEn::MR, [0b00,0b01,0b10,0b11]), 
    //0x3B => ins3!("cmp",  [0x3B], ["/r"],       OpEn::RM, [0b00,0b01,0b10,0b11]), 
    //0xFF => ins3!("dec",  [0xFF], ["/1"],  OpEn::M, [0b00,0b01,0b10,0b11]), 
    //0x48 => ins1!("dec",  [0x48], ["+rd"], OpEn::O), 
    //0xF7 => ins3!("idiv",  [0xF7], ["/7"], OpEn::M, [0b00,0b01,0b10,0b11]), 
    //0xFF => ins3!("inc",  [0xFF], ["/0"],  OpEn::M, [0b00,0b01,0b10,0b11]), 
    //0x40 => ins1!("inc",  [0x40], ["+rd"], OpEn::O),
    //0xEB => ins1!("jmp",  [0xEB], ["ib"], OpEn::D),
    //0xE9 => ins1!("jmp",  [0xE9], ["id"], OpEn::D),
    //0xFF => ins3!("jmp",  [0xFF], ["/4"], OpEn::M, [0b00,0b01,0b10,0b11]),
    //0x74 => ins1!("jz",  [0x74],       ["ib"], OpEn::D),
    //0x0F => ins1!("jz",  [0x0F, 0x84], ["id"], OpEn::D),
    //0x75 => ins1!("jnz", [0x75],       ["ib"], OpEn::D),
    //0x0F => ins1!("jnz", [0x0F, 0x85], ["id"], OpEn::D),
    //0x8D => ins3!("lea",  [0x8D], ["/r"], OpEn::RM, [0b00,0b01,0b10]),
    //0xA1 => ins0!("mov",   [0xA1],                OpEn::FD),
    //0xA3 => ins0!("mov",   [0xA3],                OpEn::TD),
    //0xB8 => ins1!("mov",   [0xB8], ["+rd", "id"], OpEn::OI),
    //0xC7 => ins3!("mov",   [0xC7], ["/0", "id"],  OpEn::MI, [0b00,0b01,0b10,0b11]),
    //0x89 => ins3!("mov",   [0x89], ["/r"],        OpEn::MR, [0b00,0b01,0b10,0b11]),
    //0x8B => ins3!("mov",   [0x8B], ["/r"],        OpEn::RM, [0b00,0b01,0b10,0b11]),
    //0xA5 => ins0!("movsd", [0xA5],                OpEn::ZO),
    //0x90 => ins0!("nop",  [0x90],              OpEn::ZO),
    //0xF7 => ins3!("not", [0xF7], ["/2"], OpEn::M, [0b00,0b01,0b10,0b11]),
    //0x0D => ins1!("or", [0x0D], ["id"],       OpEn::I),
    //0x81 => ins3!("or", [0x81], ["/1", "id"], OpEn::MI, [0b00,0b01,0b10,0b11]),
    //0x09 => ins3!("or", [0x09], ["/r"],       OpEn::MR, [0b00,0b01,0b10,0b11]),
    //0x0B => ins3!("or", [0x0B], ["/r"],       OpEn::RM, [0b00,0b01,0b10,0b11]),
    //0x8F => ins3!("pop", [0x8F], ["/0"],  OpEn::M, [0b00,0b01,0b10,0b11]),
    //0x58 => ins1!("pop", [0x58], ["+rd"], OpEn::O),
    //0xFF => ins3!("push", [0xFF], ["/6"],  OpEn::M, [0b00,0b01,0b10,0b11]),
    //0x50 => ins1!("push", [0x50], ["+rd"], OpEn::O),
    //0x68 => ins1!("push", [0x68], ["id"],  OpEn::I),
    //0x6A => ins1!("push", [0x6A], ["ib"],  OpEn::I),
    //0xF2 => ins2!("repne cmpsd", 0xF2, [0xA7], OpEn::ZO),
    //0xCB => ins0!("retf", [0xCB],         OpEn::ZO),
    //0xCA => ins1!("retf", [0xCA], ["iw"], OpEn::I),
    //0xC3 => ins0!("retn", [0xC3],         OpEn::ZO),
    //0xC2 => ins1!("retn", [0xC2], ["iw"], OpEn::I),
    //0x2D => ins1!("sub", [0x2D], ["id"],       OpEn::I),
    //0x81 => ins3!("sub", [0x81], ["/5", "id"], OpEn::MI, [0b00,0b01,0b10,0b11]),
    //0x29 => ins3!("sub", [0x29], ["/r"],       OpEn::MR, [0b00,0b01,0b10,0b11]),
    //0x2B => ins3!("sub", [0x2B], ["/r"],       OpEn::RM, [0b00,0b01,0b10,0b11]),
    //0xA9 => ins1!("test", [0xA9], ["id"],      OpEn::I),
    //0xF7 => ins3!("test", [0xF7], ["/0","id"], OpEn::MI, [0b00,0b01,0b10,0b11]),
    //0x85 => ins3!("test", [0x85], ["/r"],      OpEn::MR, [0b00,0b01,0b10,0b11]),
    //0x35 => ins1!("xor", [0x35], ["id"], OpEn::I),
    //0x81 => ins3!("xor", [0x81], ["/6"], OpEn::MI, [0b00,0b01,0b10,0b11]),
    //0x31 => ins3!("xor", [0x31], ["/r"], OpEn::MR, [0b00,0b01,0b10,0b11]),
    //0x33 => ins3!("xor", [0x33], ["/r"], OpEn::RM, [0b00,0b01,0b10,0b11]),
//};



/// Add
/// Instruction        Opcode       Addressing
/// add eax, imm32     0x05 id     MODR/M Not Required
/// add r/m32, imm32   0x81 /0 id  00/01/10/11
/// add r/m32, r32     0x01 /r     00/01/10/11
/// add r32, r/m32     0x03 /r     00/01/10/11
static ADD: Map<u8, DecodeRule> = phf_map! {
    0x05 => ins1!("add",  [0x05], ["id"],       OpEn::I),
    0x81 => ins3!("add",  [0x81], ["/0", "id"], OpEn::MI, [0b00,0b01,0b10,0b11]),
    0x01 => ins3!("add",  [0x01], ["/r"],       OpEn::MR, [0b00,0b01,0b10,0b11]),
    0x03 => ins3!("add",  [0x03], ["/r"],       OpEn::RM, [0b00,0b01,0b10,0b11]),
};


/// And
/// Instruction        Opcode       Addressing
/// and eax, imm32     0x25 id      MODR/M Not Required
/// and r/m32, imm32   0x81 /4 id   00/01/10/11
/// and r/m32, r32     0x21 /r      00/01/10/11
/// and r32, r/m32     0x23 /r      00/01/10/11
static AND: Map<u8, DecodeRule> = phf_map! {
    0x25 => ins1!("and",  [0x25], ["id"],       OpEn::I),
    0x81 => ins3!("and",  [0x81], ["/4", "id"], OpEn::MI, [0b00,0b01,0b10,0b11]),
    0x21 => ins3!("and",  [0x21], ["/r"],       OpEn::MR, [0b00,0b01,0b10,0b11]),
    0x23 => ins3!("and",  [0x23], ["/r"],       OpEn::RM, [0b00,0b01,0b10,0b11]),
};
    

/// Call
/// Instruction     Opcode     Addressing
/// call rel32      0xE8       cd Note: treat cd as id What does this mean?
/// call r/m32      0xFF /2    00/01/10/11
static CALL: Map<u8, DecodeRule> = phf_map! {
    0xE8 => ins1!("call",  [0xE8], ["id"], OpEn::D), // Is this right?
    0xFF => ins3!("call",  [0xFF], ["/2"], OpEn::M, [0b00,0b01,0b10,0b11]),
};


/// Cache Line Flush ???
/// Instruction        Opcode         Addressing
/// clflush m8         0x0F 0xAE /7   00/01/10
/// Note: m8 can be treated as r/m32 except that addressing mode 11 is illegal.
static CLFLUSH: Map<u8, DecodeRule> = phf_map! {
    0xFF => ins3!("clflush",  [0xAE], ["/7"], OpEn::M, [0b00,0b01,0b10,]), // ???
};


/// Cmp
/// Instruction        Opcode      Addressing
/// cmp eax, imm32     0x3D id      MODR/M Not Required
/// cmp r/m32, imm32   0x81 /7 id   00/01/10/11
/// cmp r/m32, r32     0x39 /r      00/01/10/11
/// cmp r32, r/m32     0x3B /r      00/01/10/11
static CMP: Map<u8, DecodeRule> = phf_map! {
    0x3D => ins1!("cmp",  [0x3D], ["/id"],      OpEn::I), 
    0x81 => ins3!("cmp",  [0x81], ["/7", "id"], OpEn::MI, [0b00,0b01,0b10,0b11]), 
    0x39 => ins3!("cmp",  [0x39], ["/r"],       OpEn::MR, [0b00,0b01,0b10,0b11]), 
    0x3B => ins3!("cmp",  [0x3B], ["/r"],       OpEn::RM, [0b00,0b01,0b10,0b11]), 
};


/// Dec
/// Instruction    Opcode      Addressing
/// dec r/m32      0xFF /1     00/01/10/11
/// dec r32        0x48 + rd   MODR/M Not Required
static DEC: Map<u8, DecodeRule> = phf_map! {
    0xFF => ins3!("dec",  [0xFF], ["/1"],  OpEn::M, [0b00,0b01,0b10,0b11]), 
    0x48 => ins1!("dec",  [0x48], ["+rd"], OpEn::O), 
};


/// Div
/// idiv r/m32 0xF7 /7 00/01/10/11
static IDIV: Map<u8, DecodeRule> = phf_map! {
    0xF7 => ins3!("idiv",  [0xF7], ["/7"], OpEn::M, [0b00,0b01,0b10,0b11]), 
};


/// Inc
/// Instruction    Opcode      Addressing
/// inc r/m32      0xFF /0     00/01/10/11
/// inc r32        0x40 + rd   MODR/M Not Required
static INC: Map<u8, DecodeRule> = phf_map! {
    0xFF => ins3!("inc",  [0xFF], ["/0"],  OpEn::M, [0b00,0b01,0b10,0b11]), 
    0x40 => ins1!("inc",  [0x40], ["+rd"], OpEn::O),
};


/// Jmp
/// Instruction    Opcode      Addressing
/// jmp rel8       0xEB cb     Note: treat cb as ib
/// jmp rel32      0xE9 cd     Note: treat cd as id
/// jmp r/m32      0xFF /4     00/01/10/11
static JMP: Map<u8, DecodeRule> = phf_map! {
    0xEB => ins1!("jmp",  [0xEB], ["ib"], OpEn::D),
    0xE9 => ins1!("jmp",  [0xE9], ["id"], OpEn::D),
    0xFF => ins3!("jmp",  [0xFF], ["/4"], OpEn::M, [0b00,0b01,0b10,0b11]),
};


/// Jmp & Cmp
/// Instruction    Opcode         Addressing
/// jz rel8        0x74 cb        Note: treat cb as ib
/// jz rel32       0x0f 0x84 cd   Note: treat cd as id
/// jnz rel8       0x75 cb        Note: treat cb as ib
/// jnz rel32      0x0f 0x85 cd   Note: treat cd as id
static JCMP: Map<u8, DecodeRule> = phf_map! {
    0x74 => ins1!("jz",  [0x74],       ["ib"], OpEn::D),
    0x0F => ins1!("jz",  [0x0F, 0x84], ["id"], OpEn::D),
    0x75 => ins1!("jnz", [0x75],       ["ib"], OpEn::D),
    0x0F => ins1!("jnz", [0x0F, 0x85], ["id"], OpEn::D),
};


/// Lea
/// Instruction        Opcode       Addressing
/// lea r32, m         0x8D /r      00/01/10
/// Note: m can be treated as r/m32 except that
/// addressing mode 11 is illegal.
static LEA: Map<u8, DecodeRule> = phf_map! {
    0x8D => ins3!("lea",  [0x8D], ["/r"], OpEn::RM, [0b00,0b01,0b10]),
};


/// Mov
/// Instruction        Opcode       Addressing
/// mov eax, moffs32   A1           Note: treat moffs32 as imm32
/// mov moffs32, eax   A3           Note: treat moffs32 as imm32
/// mov r32, imm32     0xB8+rd id   MODR/M Not Required
/// mov r/m32, imm32   0xC7 /0 id   00/01/10/11
/// mov r/m32, r32     0x89 /r      00/01/10/11
/// mov r32, r/m32     0x8B /r      00/01/10/11
/// movsd              0xA5         MODR/M Not Required
static MOV: Map<u8, DecodeRule> = phf_map! {
    0xA1 => ins0!("mov",   [0xA1],                OpEn::FD),
    0xA3 => ins0!("mov",   [0xA3],                OpEn::TD),
    0xB8 => ins1!("mov",   [0xB8], ["+rd", "id"], OpEn::OI),
    0xC7 => ins3!("mov",   [0xC7], ["/0", "id"],  OpEn::MI, [0b00,0b01,0b10,0b11]),
    0x89 => ins3!("mov",   [0x89], ["/r"],        OpEn::MR, [0b00,0b01,0b10,0b11]),
    0x8B => ins3!("mov",   [0x8B], ["/r"],        OpEn::RM, [0b00,0b01,0b10,0b11]),
    0xA5 => ins0!("movsd", [0xA5],                OpEn::ZO),
};


/// NoOp
/// Instruction  Opcode    Addressing
/// nop          0x90      MODR/M Not Required
/// Note: this is really xchg eax, eax
static NOP: Map<u8, DecodeRule> = phf_map! {
    0x90 => ins0!("nop",  [0x90],              OpEn::ZO),
};


/// Not
/// Instruction       Opcode       Addressing
/// not r/m32 0xF7 /2 00/01/10/11
static NOT: Map<u8, DecodeRule> = phf_map! {
    0xF7 => ins3!("not", [0xF7], ["/2"], OpEn::M, [0b00,0b01,0b10,0b11]),
};


/// Or
/// Instruction       Opcode       Addressing
/// or eax, imm32     0x0D id      MODR/M Not Required
/// or r/m32, imm32   0x81 /1 id   00/01/10/11
/// or r/m32, r32     0x09 /r      00/01/10/11
/// or r32, r/m32     0x0B /r      00/01/10/11
static OR: Map<u8, DecodeRule> = phf_map! {
    0x0D => ins1!("or", [0x0D], ["id"],       OpEn::I),
    0x81 => ins3!("or", [0x81], ["/1", "id"], OpEn::MI, [0b00,0b01,0b10,0b11]),
    0x09 => ins3!("or", [0x09], ["/r"],       OpEn::MR, [0b00,0b01,0b10,0b11]),
    0x0B => ins3!("or", [0x0B], ["/r"],       OpEn::RM, [0b00,0b01,0b10,0b11]),
};


/// Pop
/// Instruction    Opcode       Addressing
/// pop r/m32      0x8F /0      00/01/10/11
/// pop r32        0x58 + rd    MODR/M Not Required
static POP: Map<u8, DecodeRule> = phf_map! {
    0x8F => ins3!("pop", [0x8F], ["/0"],  OpEn::M, [0b00,0b01,0b10,0b11]),
    0x58 => ins1!("pop", [0x58], ["+rd"], OpEn::O),
};


/// Push
/// Instruction    Opcode       Addressing
/// push r/m32    0xFF /6     00/01/10/11
/// push r32      0x50 + rd   MODR/M Not Required
/// push imm32    0x68 id     MODR/M Not Required
/// push imm8     0x6a ib     MODR/M Not Required
static PUSH: Map<u8, DecodeRule> = phf_map! {
    0xFF => ins3!("push", [0xFF], ["/6"],  OpEn::M, [0b00,0b01,0b10,0b11]),
    0x50 => ins1!("push", [0x50], ["+rd"], OpEn::O),
    0x68 => ins1!("push", [0x68], ["id"],  OpEn::I),
    0x6A => ins1!("push", [0x6A], ["ib"],  OpEn::I),
};


/// Repeat and Cmp
/// Instruction    Opcode       Addressing
/// repne cmpsd    0xF2 0xA7    MODR/M Not Required
/// Note: 0xF2 is the repne prefix
static REPNECMPSD: Map<u8, DecodeRule> = phf_map! {
    0xF2 => ins2!("repne cmpsd", 0xF2, [0xA7], OpEn::ZO),
};


/// Ret
/// Instruction    Opcode       Addressing
/// retf           0xCB      MODR/M Not Required
/// retf imm16     0xCA iw   MODR/M Not Required
/// retn           0xC3      MODR/M Not Required
/// retn imm16     0xC2 iw   MODR/M Not Required
static RET: Map<u8, DecodeRule> = phf_map! {
    0xCB => ins0!("retf", [0xCB],         OpEn::ZO),
    0xCA => ins1!("retf", [0xCA], ["iw"], OpEn::I),
    0xC3 => ins0!("retn", [0xC3],         OpEn::ZO),
    0xC2 => ins1!("retn", [0xC2], ["iw"], OpEn::I),
};

    0x0D => ins1!("or", [0x0D], ["id"],       OpEn::I),
    0x81 => ins3!("or", [0x81], ["/1", "id"], OpEn::MI, [0b00,0b01,0b10,0b11]),
    0x09 => ins3!("or", [0x09], ["/r"],       OpEn::MR, [0b00,0b01,0b10,0b11]),
    0x0B => ins3!("or", [0x0B], ["/r"],       OpEn::RM, [0b00,0b01,0b10,0b11]),
    0x8F => ins3!("pop", [0x8F], ["/0"],  OpEn::M, [0b00,0b01,0b10,0b11]),
    0x58 => ins1!("pop", [0x58], ["+rd"], OpEn::O),
    0xFF => ins3!("push", [0xFF], ["/6"],  OpEn::M, [0b00,0b01,0b10,0b11]),
    0x50 => ins1!("push", [0x50], ["+rd"], OpEn::O),
    0x68 => ins1!("push", [0x68], ["id"],  OpEn::I),
    0x6A => ins1!("push", [0x6A], ["ib"],  OpEn::I),
    0xF2 => ins2!("repne cmpsd", 0xF2, [0xA7], OpEn::ZO),
    0xCB => ins0!("retf", [0xCB],         OpEn::ZO),
    0xCA => ins1!("retf", [0xCA], ["iw"], OpEn::I),
    0xC3 => ins0!("retn", [0xC3],         OpEn::ZO),
    0xC2 => ins1!("retn", [0xC2], ["iw"], OpEn::I),

/// Sub
/// Instruction         Opcode       Addressing
/// sub eax, imm32     0x2D id      MODR/M Not Required
/// sub r/m32, imm32   0x81 /5 id   00/01/10/11
/// sub r/m32, r32     0x29 /r      00/01/10/11
/// sub r32, r/m32     0x2B /r      00/01/10/11
static SUB: Map<u8, DecodeRule> = phf_map! {
    0x2D => ins1!("sub", [0x2D], ["id"],       OpEn::I),
    0x81 => ins3!("sub", [0x81], ["/5", "id"], OpEn::MI, [0b00,0b01,0b10,0b11]),
    0x29 => ins3!("sub", [0x29], ["/r"],       OpEn::MR, [0b00,0b01,0b10,0b11]),
    0x2B => ins3!("sub", [0x2B], ["/r"],       OpEn::RM, [0b00,0b01,0b10,0b11]),
};


/// Test
/// Instruction         Opcode       Addressing
/// test eax, imm32     0xA9 id      MODR/M Not Required
/// test r/m32, imm32   0xF7 /0 id   00/01/10/11
/// test r/m32, r32     0x85 /r      00/01/10/11
static TEST: Map<u8, DecodeRule> = phf_map! {
    0xA9 => ins1!("test", [0xA9], ["id"],      OpEn::I),
    0xF7 => ins3!("test", [0xF7], ["/0","id"], OpEn::MI, [0b00,0b01,0b10,0b11]),
    0x85 => ins3!("test", [0x85], ["/r"],      OpEn::MR, [0b00,0b01,0b10,0b11]),
};


/// Xor
/// Instruction         Opcode       Addressing
/// xor eax, imm32     0x35 id      MODR/M Not Required
/// xor r/m32, imm32   0x81 /6 id   00/01/10/11
/// xor r/m32, r32     0x31 /r      00/01/10/11
/// xor r32, r/m32     0x33 /r      00/01/10/11
static XOR: Map<u8, DecodeRule> = phf_map! {
    0x35 => ins1!("xor", [0x35], ["id"], OpEn::I),
    0x81 => ins3!("xor", [0x81], ["/6"], OpEn::MI, [0b00,0b01,0b10,0b11]),
    0x31 => ins3!("xor", [0x31], ["/r"], OpEn::MR, [0b00,0b01,0b10,0b11]),
    0x33 => ins3!("xor", [0x33], ["/r"], OpEn::RM, [0b00,0b01,0b10,0b11]),
};
