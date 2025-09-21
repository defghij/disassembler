
#[cfg(test)]
pub mod files {
    use std::path::Path;

    use tracing_test::traced_test;
    use tracing::{info, error, debug};

    use crate::{instruction::encoding::{operands::Operand, Sib}, output::Disassembly};
    #[allow(unused)]
    use crate::{
        output::setup_tracing,
        decode::{
            Bytes, DecodeRule
        }, instruction::{encoding::operands::Offset, Instruction}, opcodes::DecodeRules,
    };
    use crate::input::get_bytes;

    #[test]
    fn example1() {
        let bytes = include_bytes!("./example1").to_vec();
        let expected_raw = include_str!("./example1.out");
        let expected: Vec<char> = expected_raw
            .chars()
            .filter(|c|{
                !(c.is_whitespace() || c.is_control())
            }).collect();

        println!("{bytes:X?}");
        
        let output = Disassembly::from(bytes);

        
        println!("Output:\n{output}");
        println!("\nExpected:\n{expected_raw}");
        let output: Vec<char> = format!("{output}")
            .chars().filter(|c|!(c.is_whitespace() || c.is_control())).collect();

        // Just test raw characters independent of spacing and newlines
        output.iter().zip(expected.iter()).for_each(|(a,b)| {println!("{a} {b}"); assert!( a == b);});
    }

}
 
//This contains tests from the course 
#[cfg(test)]
pub mod compendium {
    use tracing_test::traced_test;
    use tracing::{info, error, debug};

    use crate::{instruction::encoding::{operands::Operand, Sib}, output::Disassembly};
    #[allow(unused)]
    use crate::{
        output::setup_tracing,
        decode::{
            Bytes, DecodeRule
        }, instruction::{encoding::operands::Offset, Instruction}, opcodes::DecodeRules,
    };

    #[traced_test]
    fn check(expected: String, bytes: &[u8]) {
        assert!(bytes.len() >= 1);

        //let mut output = Disassembly::new(10);
        //let offset = Offset(0); // All test instructions start at Address Zero
        //let Ok(rules) = DecodeRules::get(&bytes[0]) 
            //else { error!("Encountered unexpected Opcode"); panic!() };

        //info!("");
        //info!("");
        //info!("Checking\n:{expected}");
        //info!("----------------------------------------");
        
        //for rule in rules { // We dont know which rule will decode into an instruction
            //info!("");
            //info!("Attempting Decode using rule: {rule:?}");
            //info!("----------------------------------------");

            //let (mut length, _fixed) = rule.len();
            //debug!("Rule reported opcode length: {length}");

            //let base = offset.0 as usize;

            //let instruction = if rule.modrm_required() { // We must decode bytes beyond the first to determine length
                //debug!("ModRM required for instruction decode");

                //let modrm_location = rule.op_code().len();
                //let Ok(modrm) = rule.modrm_byte(bytes[modrm_location]) else { continue };
                //debug!("Got ModRM Byte: 0x{:X} = {modrm:?}", modrm.as_byte());

                //let sib = if modrm.precedes_sib_byte() {
                    //debug!("Attempting decode of SIB byte from 0x{:X}", bytes[modrm_location+1]);
                    //let sib = Sib::try_from(bytes[modrm_location+1]);
                    //if sib.is_err() { continue; } 
                    //else { sib.ok() }
                //} else { None };

                //let Ok(bytes_remaining) = modrm.bytes_remaining(sib)
                    //else { continue; };
                //length += bytes_remaining;

                //debug!("Instuction length updated: {length}");
                //debug!("Grabbing byte range {:?} for decode attempt", (base..base+length));

                //let Some(prospective_bytes) = bytes.get(base.. base + length)
                       //else { error!("Test should have enough bytes for decoding instruction"); panic!() };

                //let decode_attempt = Bytes::from(offset.clone(), prospective_bytes, rule.clone());

                //let instruction = if decode_attempt.is_ok() { decode_attempt.expect("Ok due to conditional") }
                    //else { info!("Decode unsuccessful"); continue };
                
                //instruction
            //} 
            //else { // We can know the length of the instruction _a priori_

                //debug!("Rule reported true byte length: {length}");

                //let Some(prospective_bytes) = bytes.get(0..length)
                    //else { error!("Test should have enough bytes for decoding instruction"); panic!() };

                //let decode_attempt = Bytes::from(offset.clone(), prospective_bytes, rule.clone());

                //let instruction = if decode_attempt.is_ok() { decode_attempt.expect("Ok due to conditional") }
                    //else { info!("Decode unsuccessful"); continue };

                //instruction
            //};

            //info!("Instruction\n:{instruction:?}");

            //if instruction.decoded_successfully() {
                //info!("Decoded Instruction\n:{instruction:?}");
                //output.add(instruction.clone())
                    //.expect("This manually decoded instruction should be valid");
                //if rule.can_make_label() {
                        //let instruction = instruction
                            //.get_instruction().expect("Should be a valid instruction");

                    //let has_label_operand = instruction.operands.iter().any(|op| matches!(op, Operand::Label(_)));

                    //if has_label_operand {
                        //let label = instruction
                            //.get_displacement_offset().expect("Should have label");

                        //let _ = output.label(label); // Dont worry about the result in a test. We'll
                                                     //// regularly add labels "beyond" range
                    //}
                //}
                //break; 
            //}
        //}
        let output = Disassembly::from(bytes.to_vec());
        assert_eq!(output.to_string(), expected);
    }

    #[test]
    fn zero() {
        let mapping: Vec<(&str, &[u8])> = vec![
            ("00000000: CC     int3", &[0xCC]),// not required by assignment
            ("00000000: 99     cdq",  &[0x99]), // not require by assignment
            ("00000000: C3     retn", &[0xC3]),
            ("00000000: CB     retf", &[0xCB]),
        ];
        mapping.iter()
            .for_each(|(s,b)| { check(s.to_string(),b); });
    }

    #[test]
    fn opcode() {
        let mapping: Vec<(&str, &[u8])> = vec![
            ("00000000: 4B     dec ebx",  &[0x4B]),
            ("00000000: 40     inc eax",  &[0x40]),
            ("00000000: 48     dec eax",  &[0x48]),
            ("00000000: 51     push ecx", &[0x51]),
            ("00000000: 5F     pop edi",  &[0x5F]),
        ];
        mapping.iter()
            .for_each(|(s,b)| { check(s.to_string(),b); });
    }

    #[test]
    fn immediate() {
        let mapping: Vec<(&str, &[u8])> = vec![
            ("00000000: 68 DD CC BB AA     push 0xAABBCCDD",     &[0x68, 0xDD, 0xCC, 0xBB, 0xAA]),
            ("00000000: CD 03     int 0x03",                     &[0xCD, 0x03                  ]),
            ("00000000: 05 DD CC BB AA     add eax, 0xAABBCCDD", &[0x05, 0xDD, 0xCC, 0xBB, 0xAA]),
        ];
        mapping.iter()
            .for_each(|(s,b)| { check(s.to_string(),b); });
    }

    #[test]
    fn displacement() {
        let mapping: Vec<(&str, &[u8])> = vec![
            // Disassembly from nasm and objdump as baseline for test.
            //0:   74 0f                   je     0x11
            ("00000000: 74 0F     jz offset_00000011h",        &[0x74, 0x0F]),

            //0:   75 0f                   jne    0x11
            ("00000000: 75 0F     jnz offset_00000011h",        &[0x75, 0x0F]),

            //0:   0f 84 d9 cc bb aa       je     0xaabbccdf
            ("00000000: 0F 84 D9 CC BB AA     jz offset_AABBCCDFh", &[0x0F, 0x84, 0xD9, 0xCC, 0xBB, 0xAA]),

            //0:   e8 d8 cc bb aa          call   0xaabbccdd
            ("00000000: E8 D8 CC BB AA     call offset_AABBCCDDh", &[0xE8, 0xD8, 0xCC, 0xBB, 0xAA]),

            //0:   e8 06 00 00 00          call   0xb
            ("00000000: E8 06 00 00 00     call offset_0000000Bh", &[0xE8, 0x06, 0x00, 0x00, 0x00]),
        ];
        mapping.iter()
            .for_each(|(s,b)| { check(s.to_string(),b); });
    }

    #[test]
    fn opcode_and_immediate() {
        let mapping: Vec<(&str, &[u8])> = vec![
            ("00000000: BB DD CC BB AA     mov ebx, 0xAABBCCDD", &[0xBB, 0xDD, 0xCC, 0xBB, 0xAA]),
        ];
        mapping.iter()
            .for_each(|(s,b)| { check(s.to_string(),b); });
    }

    #[test]
    fn m_rm() {
        let mapping: Vec<(&str, &[u8])> = vec![
            ("00000000: FF F1     push ecx", 
             &[0xFF, 0xF1]),
             
            ("00000000: FF 35 DD CC BB AA     push [ 0xAABBCCDD ]", 
             &[0xFF, 0x35, 0xDD, 0xCC, 0xBB, 0xAA]),

            ("00000000: FF 0B     dec [ ebx ]", 
             &[0xFF, 0x0B]),

            ("00000000: FF 40 30     inc [ eax + 0x00000030 ]", 
             &[0xFF, 0x40, 0x30]),

            ("00000000: FF B6 DD CC BB AA     push [ esi + 0xAABBCCDD ]", 
             &[0xFF, 0xB6, 0xDD, 0xCC, 0xBB, 0xAA]),

            ("00000000: FF 84 03 DD CC BB AA     inc [ eax + ebx + 0xAABBCCDD ]", 
             &[0xFF, 0x84, 0x03, 0xDD, 0xCC, 0xBB, 0xAA]),

            ("00000000: FF 34 35 DD CC BB AA     push [ esi + 0xAABBCCDD ]", 
             &[0xFF, 0x34, 0x35, 0xDD, 0xCC, 0xBB, 0xAA]),

            ("00000000: FF 14 24     call [ esp ]", 
             &[0xFF, 0x14, 0x24                        ]),
        ];
        mapping.iter()
            .for_each(|(s,b)| { check(s.to_string(),b); });
    }

    #[test]
    fn m1_rm_and_one() {
        let mapping: Vec<(&str, &[u8])> = vec![
            ("00000000: D1 7B 10     sar [ ebx + 0x00000010 ], 0x01",
             &[0xD1, 0x7B, 0x10]),

            ("00000000: D1 24 56     sal [ edx * 2 + esi ], 0x01",
             &[0xD1, 0x24, 0x56]),

            ("00000000: D1 2C 7D DD CC BB AA     shr [ edi * 2 + 0xAABBCCDD ], 0x01",
             &[0xD1, 0x2C, 0x7D, 0xDD, 0xCC, 0xBB, 0xAA]),
        ];
        mapping.iter()
            .for_each(|(s,b)| { check(s.to_string(),b); });
    }

    #[test]
    fn mi_rm_and_immediate() {
        let mapping: Vec<(&str, &[u8])> = vec![
            ("00000000: 81 CF DD CC BB AA     or edi, 0xAABBCCDD",
             &[0x81, 0xCF, 0xDD, 0xCC, 0xBB, 0xAA]),

            ("00000000: 81 75 00 DD CC BB AA     xor [ ebp ], 0xAABBCCDD",
             &[0x81, 0x75, 0x00, 0xDD, 0xCC, 0xBB, 0xAA]),
        ];
        mapping.iter()
            .for_each(|(s,b)| { check(s.to_string(),b); });
    }

    #[test]
    fn mr_rm_and_reg() {
        let mapping: Vec<(&str, &[u8])> = vec![
            ("00000000: 39 FE     cmp esi, edi",
             &[0x39, 0xFE]),

            ("00000000: 31 35 DD CC BB AA     xor [ 0xAABBCCDD ], esi",
             &[0x31, 0x35, 0xDD, 0xCC, 0xBB, 0xAA]),

            ("00000000: 01 8F DD CC BB AA     add [ edi + 0xAABBCCDD ], ecx",
             &[0x01, 0x8F, 0xDD, 0xCC, 0xBB, 0xAA]),

            ("00000000: 89 BC B3 DD CC BB AA     mov [ esi * 4 + ebx + 0xAABBCCDD ], edi",
             &[0x89, 0xBC, 0xB3, 0xDD, 0xCC, 0xBB, 0xAA]),

            ("00000000: 39 04 9D DD CC BB AA     cmp [ ebx * 4 + 0xAABBCCDD ], eax",
             &[0x39, 0x04, 0x9D, 0xDD, 0xCC, 0xBB, 0xAA]),
        ];
        mapping.iter()
            .for_each(|(s,b)| { check(s.to_string(),b); });
    }

    #[test]
    fn rm_reg_and_rm() {
        let mapping: Vec<(&str, &[u8])> = vec![
            ("00000000: 3B FE     cmp edi, esi",
             &[0x3B, 0xFE]),

            ("00000000: 03 8F DD CC BB AA     add ecx, [ edi + 0xAABBCCDD ]",
             &[0x03, 0x8F, 0xDD, 0xCC, 0xBB, 0xAA]),

            ("00000000: 8B BC B3 DD CC BB AA     mov edi, [ esi * 4 + ebx + 0xAABBCCDD ]",
             &[0x8B, 0xBC, 0xB3, 0xDD, 0xCC, 0xBB, 0xAA]),
             
            ("00000000: 3B 04 9D DD CC BB AA     cmp eax, [ ebx * 4 + 0xAABBCCDD ]",
             &[0x3B, 0x04, 0x9D, 0xDD, 0xCC, 0xBB, 0xAA]),
        ];
        mapping.iter()
            .for_each(|(s,b)| { check(s.to_string(),b); });
    }

    #[test]
    fn rmi_regrm_and_immediate() {
        let mapping: Vec<(&str, &[u8])> = vec![
            ("00000000: 6B C2 10     imul eax, edx, 0x10", 
             &[0x6B, 0xC2, 0x10]),

            ("00000000: 69 BC F0 DD CC BB AA 44 33 22 11     imul edi, [ esi * 8 + eax + 0xAABBCCDD ], 0x11223344",
             &[0x69, 0xBC, 0xF0, 0xDD, 0xCC, 0xBB, 0xAA, 0x44, 0x33, 0x22, 0x11]), 

            ("00000000: 69 1C C5 DD CC BB AA 44 33 22 11     imul ebx, [ eax * 8 + 0xAABBCCDD ], 0x11223344",
             &[0x69, 0x1C, 0xC5, 0xDD, 0xCC, 0xBB, 0xAA, 0x44, 0x33, 0x22, 0x11]),
        ];
        mapping.iter()
            .for_each(|(s,b)| { check(s.to_string(),b); });
    }

    #[test]
    fn misc_instructions() {
        let mapping: Vec<(&str, &[u8])> = vec![
            ("00000000: B8 44 43 42 41     mov eax, 0x41424344",
             &[0xB8, 0x44, 0x43, 0x42,0x41]),
            ("00000000: 8B 95 08 00 00 00     mov edx, [ ebp + 0x00000008 ]",
             &[0x8B, 0x95, 0x08, 0x00, 0x00, 0x00])
        ];
        mapping.iter()
            .for_each(|(s,b)| { check(s.to_string(),b); });
    }
}
