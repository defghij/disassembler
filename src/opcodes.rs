use phf::{phf_map, Map};

use crate::instruction::encoding::ModRM;

use super::{
    decode::{
        DecodeRule,
        DecodeError,
    },
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

/// Returns a Vector of possible ways to decode the current and
/// potentially follow on bytes based _only_ on a single byte.
///
/// *Assumption*:
/// Relies on the assumption that the [`Map<u8,DecodeRule>`]s contained
/// in OPCODES has a one-to-one relationship between key,value pairs.
//pub fn _presumptive_decode_rules(key: u8) -> &'static[DecodeRule] {
    //let _dc_rules = DECODE_RULES.get(&key);

    //unimplemented!("lol");
//}



macro_rules! ins0 {
    ($Mnemonic:literal, $OpCodes:expr, $OpEn:expr) => {
        DecodeRule($Mnemonic, 
                   None, 
                   OpCode(&$OpCodes), 
                   None,  
                   $OpEn, 
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
                   $OpEn, 
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
                   $OpEn, 
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
                   $OpEn, 
                   Some(AddressingModes(&$AddrModes))
        )
    };
}

pub struct DecodeRules(RulesMap);
impl DecodeRules {
    pub fn get(byte: &u8) -> Result<Rules, DecodeError> {
        match DECODE_RULES.get(&format!("0x{byte:02X}")) {
            Some(rules) => Ok(*rules),
            None => Err(DecodeError::UnknownOpcode)
        }
    }
}
#[test]
fn access_internal_state() {
    let rules = DecodeRules::get(&0x01);
    assert!(
        rules.is_ok_and(|rules| {
            assert!(rules.len() == 1);
            let rule = rules.get(0)
                .expect("Should be only one element");
            rule.0 == "add"
        })
    );
}

#[test]
fn retrieve_single_decode_rule() {
    let rules = DecodeRules::get(&0x01);
    let expected: &'static [DecodeRule] = 
        &[ins3!("add",      [0x01],       ["/r"],       OpEn::MR, [0b00,0b01,0b10,0b11])];
    assert_eq!(rules, Ok(expected));
}

#[test]
fn retrieve_multiple_decode_rules() {
    let rules = DecodeRules::get(&0x0f);
    assert!(rules.is_ok_and(|rules| { rules.len() == 3 }));
}

#[test]
fn retrieve_nonexistent_rule() {
    let rules = DecodeRules::get(&0x00);
    assert!(rules.is_err_and(|e| e == DecodeError::UnknownOpcode));
}
                           
type Rules = &'static [DecodeRule];
type RulesMap = Map<&'static str, Rules>;

#[allow(unused)]
static DECODE_RULES: RulesMap = phf_map! {
//  Byte -->     Mnemonic       OpCode    Extensions      OpEncoding      Addressing Modes
    "0x01" => &[ins3!("add",      [0x01],       ["/r"],      OpEn::MR, [0b00,0b01,0b10,0b11])],
    "0x03" => &[ins3!("add",      [0x03],       ["/r"],      OpEn::RM, [0b00,0b01,0b10,0b11])],
    "0x05" => &[ins1!("add",      [0x05],       ["id"],      OpEn::I                        )],
    "0x09" => &[ins3!("or",       [0x09],       ["/r"],      OpEn::MR, [0b00,0b01,0b10,0b11])],
    "0x0B" => &[ins3!("or",       [0x0B],       ["/r"],      OpEn::RM, [0b00,0b01,0b10,0b11])],
    "0x0D" => &[ins1!("or",       [0x0D],       ["id"],      OpEn::I                        )],
    "0x0F" => &[ins1!("jz",       [0x0F, 0x84], ["id"],      OpEn::D                        ),
                ins1!("jnz",      [0x0F, 0x85], ["id"],      OpEn::D                        ),
                ins3!("clflush",  [0x0F, 0xAE], ["/7"],      OpEn::M, [0b00,0b01,0b10,]     ), // ???
    ],
   "0x25" => &[ins1!("and",      [0x25],       ["id"],       OpEn::I                        )],
   "0x21" => &[ins3!("and",      [0x21],       ["/r"],       OpEn::MR, [0b00,0b01,0b10,0b11])],
   "0x23" => &[ins3!("and",      [0x23],       ["/r"],       OpEn::RM, [0b00,0b01,0b10,0b11])],
   "0x29" => &[ins3!("sub",      [0x29],       ["/r"],       OpEn::MR, [0b00,0b01,0b10,0b11])],
   "0x2B" => &[ins3!("sub",      [0x2B],       ["/r"],       OpEn::RM, [0b00,0b01,0b10,0b11])],
   "0x2D" => &[ins1!("sub",      [0x2D],       ["id"],       OpEn::I                        )],
   "0x31" => &[ins3!("xor",      [0x31],       ["/r"],       OpEn::MR, [0b00,0b01,0b10,0b11])],
   "0x33" => &[ins3!("xor",      [0x33],       ["/r"],       OpEn::RM, [0b00,0b01,0b10,0b11])],
   "0x35" => &[ins1!("xor",      [0x35],       ["id"],       OpEn::I                        )],
   "0x39" => &[ins3!("cmp",      [0x39],       ["/r"],       OpEn::MR, [0b00,0b01,0b10,0b11])], 
   "0x3B" => &[ins3!("cmp",      [0x3B],       ["/r"],       OpEn::RM, [0b00,0b01,0b10,0b11])], 
   "0x3D" => &[ins1!("cmp",      [0x3D],       ["/id"],      OpEn::I                        )], 

   //EAX      ECX      EDX      EBX      ESP      EBP      ESI       EDI
   "0x40" | "0x41" | "0x42" | "0x43" | "0x44" | "0x45" | "0x46" | "0x47" => 
             &[ins1!("inc",      [0x40],       ["+rd"],      OpEn::O                        )],

   "0x48" | "0x49" | "0x4A" | "0x4B" | "0x4C" | "0x4D" | "0x4E" | "0x4F" => 
             &[ins1!("dec",      [0x48],       ["+rd"],      OpEn::O                        )], 

   "0x50" | "0x51" | "0x52" | "0x53" | "0x54" | "0x55" | "0x56" | "0x57" => 
             &[ins1!("push",     [0x50],       ["+rd"],      OpEn::O                        )],

   "0x58" | "0x59" | "0x5A" | "0x5B" | "0x5C" | "0x5D" | "0x5E" | "0x5F" => 
             &[ins1!("pop",      [0x58],       ["+rd"],      OpEn::O                        )],

   "0x68" => &[ins1!("push",     [0x68],       ["id"],       OpEn::I                        )],
   "0x6A" => &[ins1!("push",     [0x6A],       ["ib"],       OpEn::I                        )],
   "0x74" => &[ins1!("jz",       [0x74],       ["ib"],       OpEn::D                        )],
   "0x75" => &[ins1!("jnz",      [0x75],       ["ib"],       OpEn::D                        )],
   "0x81" => &[ins3!("add",      [0x81],       ["/0", "id"], OpEn::MI, [0b00,0b01,0b10,0b11]),
               ins3!("and",      [0x81],       ["/4", "id"], OpEn::MI, [0b00,0b01,0b10,0b11]),
               ins3!("cmp",      [0x81],       ["/7", "id"], OpEn::MI, [0b00,0b01,0b10,0b11]),
               ins3!("or",       [0x81],       ["/1", "id"], OpEn::MI, [0b00,0b01,0b10,0b11]),
               ins3!("sub",      [0x81],       ["/5", "id"], OpEn::MI, [0b00,0b01,0b10,0b11]),
               ins3!("xor",      [0x81],       ["/6"],       OpEn::MI, [0b00,0b01,0b10,0b11]),
    ], 
    "0x85" => &[ins3!("test",     [0x85],       ["/r"],       OpEn::MR, [0b00,0b01,0b10,0b11])],
    "0x89" => &[ins3!("mov",      [0x89],       ["/r"],       OpEn::MR, [0b00,0b01,0b10,0b11])],
    "0x8B" => &[ins3!("mov",      [0x8B],       ["/r"],       OpEn::RM, [0b00,0b01,0b10,0b11])],
    "0x8D" => &[ins3!("lea",      [0x8D],       ["/r"],       OpEn::RM, [0b00,0b01,0b10]     )],
    "0x8F" => &[ins3!("pop",      [0x8F],       ["/0"],       OpEn::M,  [0b00,0b01,0b10,0b11])],
    "0x90" => &[ins0!("nop",      [0x90],                     OpEn::ZO                       )],
    "0x99" => &[ins0!("cdq",      [0x99],                     OpEn::ZO                       )],
    "0xA1" => &[ins0!("mov",      [0xA1],                     OpEn::FD                       )],
    "0xA3" => &[ins0!("mov",      [0xA3],                     OpEn::TD                       )],
    "0xA5" => &[ins0!("movsd",    [0xA5],                     OpEn::ZO                       )],
    "0xA9" => &[ins1!("test",     [0xA9], ["id"],             OpEn::I                        )],
    "0xB8" => &[ins1!("mov",      [0xB8], ["+rd", "id"],      OpEn::OI                       )],
    "0xC2" => &[ins1!("retn",     [0xC2], ["iw"],             OpEn::I                        )],
    "0xC3" => &[ins0!("retn",     [0xC3],                     OpEn::ZO                       )],
    "0xC7" => &[ins3!("mov",      [0xC7], ["/0", "id"],       OpEn::MI, [0b00,0b01,0b10,0b11])],
    "0xCA" => &[ins1!("retf",     [0xCA], ["iw"],             OpEn::I                        )],
    "0xCB" => &[ins0!("retf",     [0xCB],                     OpEn::ZO                       )],
    "0xCC" => &[ins0!("int3",     [0xCC],                     OpEn::ZO                       )],
    "0xCD" => &[ins1!("int",      [0xCC], ["ib"],             OpEn::I                        )],
    "0xE8" => &[ins1!("call",     [0xE8], ["id"],             OpEn::D                        )], 
    "0xE9" => &[ins1!("jmp",      [0xE9], ["id"],             OpEn::D                        )],
    "0xEB" => &[ins1!("jmp",      [0xEB], ["ib"],             OpEn::D                        )],
    "0xF7" => &[ins3!("idiv",     [0xF7], ["/7"],             OpEn::M,  [0b00,0b01,0b10,0b11]), 
                ins3!("not",      [0xF7], ["/2"],             OpEn::M,  [0b00,0b01,0b10,0b11]),
                ins3!("test",     [0xF7], ["/0","id"],        OpEn::MI, [0b00,0b01,0b10,0b11]),
    ],
    "0xFF" => &[ins3!("call",     [0xFF], ["/2"],             OpEn::M,  [0b00,0b01,0b10,0b11]),
                ins3!("dec",      [0xFF], ["/1"],             OpEn::M,  [0b00,0b01,0b10,0b11]), 
                ins3!("inc",      [0xFF], ["/0"],             OpEn::M,  [0b00,0b01,0b10,0b11]), 
                ins3!("jmp",      [0xFF], ["/4"],             OpEn::M,  [0b00,0b01,0b10,0b11]),
                ins3!("push",     [0xFF], ["/6"],             OpEn::M,  [0b00,0b01,0b10,0b11]),
    ],
    // This guy breaks the table formatting even more and she's unique, it'll be fine down here
    "0xF2" => &[ins2!("repne cmpsd", 0xF2, [0xA7],            OpEn::ZO                       )],
};



