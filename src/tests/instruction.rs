 //This contains tests from the course 
#[cfg(test)]
pub mod compendium {
    #[allow(unused)]
    use crate::{
        decode::{
            Bytes, DecodeRule
        }, instruction::{encoding::operands::Offset, Instruction}, opcodes::DecodeRules, output::Output
    };

    fn check(expected: String, bytes: &[u8]) {
        assert!(bytes.len() >= 1);

        let mut output = Output::new(10);

        let offset = Offset(0); // All test instructions start at Address Zero
        println!("Checking\n:{expected}");

        let rules = DecodeRules::get(&bytes[0])
            .expect("Should be a defined opcode mapping");

        for rule in rules { // We dont know which rule will decode into an instruction

            let (length, fixed) = rule.len();
            let requires_modrm = rule.modrm_required();
            println!("rule reported modrm required: {requires_modrm}");

            let instruction = if requires_modrm { // We must decode bytes beyond the first to determine length

                let modrm = rule.modrm_byte(bytes[1]);
                if modrm.is_some() {
                    println!("ModRM byte is valid");
                    // This precedes that
                    let modrm = modrm.expect("Should be Some due to conditional");
                    
                    if modrm.precedes_sib_byte() { // Need to handle SIB Byte.
                        unimplemented!("SIB byte processing not implemented");
                    }

                    let bytes_remaining = modrm.bytes_remaining();

                    println!("reported total instruction bytes: {length} + {bytes_remaining} = {}", length + bytes_remaining); 

                    let prospective_bytes = bytes.get(0.. length + bytes_remaining)
                            .expect("Test should have enough bytes for decoding instruction");

                    let b = Bytes::from(offset.clone(), prospective_bytes, rule.clone());
                    println!("{b}");
                    b

                } else {
                    Bytes::Uknown(bytes[0])
                }
            } 
            else { // We can know the length of the instruction _a priori_
                let (length, fixed) = rule.len();
                assert!(fixed);

                println!("rule reported byte length: {length}");

                let prospective_bytes = bytes.get(0..length)
                    .expect("Test should have enough bytes for decoding instruction");

                Bytes::from(offset.clone(), prospective_bytes, rule.clone())
            };

            println!("Attempted Instruction\n:{instruction:?}");

            if instruction.decoded_successfully() {
                println!("Decoded Instruction\n:{instruction:?}");
                output.add(instruction.clone())
                    .expect("This manually decoded instruction should be valid");
                if rule.makes_label() {
                    let label = instruction
                        .get_instruction().expect("Should be a valid instruction due to the conditional above")
                        .get_displacement_offset().expect("Should be an instruction that requires a label reference due to conditional");

                    let _ = output.label(label); // Dont worry about the result in a test. We'll
                                                 // regularly add labels "beyond" range
                }
                break; 
            }
        }
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
            // Output from nasm and objdump as baseline for test.
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

            ("00000000: FF 40 30     inc [ eax + 0x30 ]", 
             &[0xFF, 0x40, 0x30]),

            ("00000000: FF 40 30    inc [ eax + 0x30 ]", 
             &[0xFF, 0x40, 0x30]),

            ("00000000: FF B6 DD CC BB AA     push [ esi + 0xAABBCCDD ]", 
             &[0xFF, 0xB6, 0xDD, 0xCC, 0xBB, 0xAA]),

            ("00000000: FF84 03 DD CC BB AA     inc [ eax + ebx + 0xAABBCCDD ]", 
             &[0xFF, 0x84, 0x03, 0xDD, 0xCC, 0xBB, 0xAA]),

            ("00000000: FF 34 35 DD CC BB AA     push [ esi + 0xAABBCCDD ]", 
             &[0xFF, 0x34, 0x35, 0xDD, 0xCC, 0xBB, 0xAA]),

            ("00000000: FF 14 24     call [ esp ]", 
             &[0xFF, 0x14, 0x24                        ]),
        ];
        mapping.iter()
            .for_each(|(s,b)| { check(s.to_string(),b); });
    }

    //#[test]
    //fn m1_rm_and_one() {                                                     8
        //let mapping: Vec<(&str, &[u8])> = vec![
            //("sar [ ebx + 0x10 ], 1",
             //&[0xD1, 0x7B, 0x10]),

            //("sal [ edx * 2 + esi ], 1",
             //&[0xD1, 0x24, 0x56]),

            //("shr [ edi * 2 + 0xAABBCCDD ], 1",
             //&[0xD1, 0x2C, 0x7D, 0xDD, 0xCC, 0xBB, 0xAA]),
        //];
        //check_instructions(mapping);
    //}

    //#[test]
    //fn mi_rm_and_immediate() {
        //let mapping: Vec<(&str, &[u8])> = vec![
            //("or edi, 0xAABBCCDD",
             //&[0x81, 0xCF, 0xDD, 0xCC, 0xBB, 0xAA]),

            //("xor [ ebp ], 0xAABBCCDD",
             //&[0x81, 0x75, 0x00, 0xDD, 0xCC, 0xBB, 0xAA]),
        //];
        //check_instructions(mapping);
    //}

    //#[test]
    //fn mr_rm_and_reg() {
        //let mapping: Vec<(&str, &[u8])> = vec![
            //("cmp esi, edi",
             //&[0x39, 0xFE]),

            //("xor [ 0xAABBCCDD ], esi",
             //&[0x31, 0x35, 0xDD, 0xCC, 0xBB, 0xAA]),

            //("add [ edi + 0xAABBCCDD ], ecx",
             //&[0x01, 0x8F, 0xDD, 0xCC, 0xBB, 0xAA]),

            //("mov [ esi * 4 + ebx + 0xAABBCCDD ], edi",
             //&[0x89, 0xBC, 0xB3, 0xDD, 0xCC, 0xBB, 0xAA]),

            //("cmp [ ebx * 4 + 0xAABBCCDD ], eax",
             //&[0x39, 0x04, 0x9D, 0xDD, 0xCC, 0xBB, 0xAA]),
        //];
        //check_instructions(mapping);
    //}

    //#[test]
    //fn rm_reg_and_rm() {
        //let mapping: Vec<(&str, &[u8])> = vec![
            //("cmp edi, esi",
             //&[0x3B, 0xFE]),

            //("dd ecx, [ edi + 0xAABBCCDD ]",
             //&[0x03, 0x8F, 0xDD, 0xCC, 0xBB, 0xAA]),

            //("mov edi, [ esi * 4 + ebx + 0xAABBCCDD ]",
             //&[0x8B, 0xBC, 0xB3, 0xDD, 0xCC, 0xBB, 0xAA]),
             
            //("cmp eax, [ ebx * 4 + 0xAABBCCDD ]",
             //&[0x3B, 0x04, 0x9D, 0xDD, 0xCC, 0xBB, 0xAA]),
        //];
        //check_instructions(mapping);
    //}

    //#[test]
    //fn rmi_regrm_and_immediate() {
        //let mapping: Vec<(&str, &[u8])> = vec![
            //("imul eax, edx, 0x10", 
             //&[0x6B, 0xC2, 0x10]),

            //("imul edi, [ esi * 8 + eax + 0xAABBCCDD ], 0x11223344",
             //&[0x69, 0xBC, 0xF0, 0xDD, 0xCC, 0xBB, 0xAA, 0x44, 0x33, 0x22, 0x11]), 

            //("imul ebx, [ eax * 8 + 0xAABBCCDD ], 0x11223344",
             //&[0x69, 0x1C, 0xC5, 0xDD, 0xCC, 0xBB, 0xAA, 0x44, 0x33, 0x22, 0x11]),
        //];
        //check_instructions(mapping);
    //}
}
