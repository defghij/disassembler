 //This contains tests from the course 
#[cfg(test)]
pub mod compendium {
    use crate::{
        opcodes::DecodeRules,
        decode::{
            DecodeRule,
            Bytes,
        },
        output::Output
    };

    fn zero_operand_full(expected: String, byte: &[u8]) {
        assert!(byte.len() == 1);

        let rules = DecodeRules::get(&byte[0])
            .expect("Should be a defined opcode mapping");
        assert!(rules.len() == 1);

        let dc_rule: &DecodeRule = rules.get(0).expect("Should be only one element");
        assert!(dc_rule.len() == 1);

        let instruction = format!("{dc_rule}");
        let bytes = dc_rule.op_code().bytes();
        let instruction = Bytes::Decoded { bytes, instruction };


        let mut output = Output::new(1);
        output.add(instruction).expect("This manually decoded instruction should be valid");
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
            .for_each(|(s,b)| { zero_operand_full(s.to_string(),b); });
    }

    fn check_opcode(expected: String, byte: &[u8]) {
        assert!(byte.len() == 1);

        let rules = DecodeRules::get(&byte[0])
            .expect("Should be a defined opcode mapping");
        assert!(rules.len() == 1);

        let rule: &DecodeRule = rules.get(0).expect("Should be only one element");
        assert!(rule.len() == 1);
        let decoded = Bytes::from(byte, rule.clone());

        let mut output = Output::new(1);
        output.add(decoded).expect("This manually decoded instruction should be valid");
        assert_eq!(output.to_string(), expected);
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
            .for_each(|(s,b)| { check_opcode(s.to_string(),b); });
    }

    fn check_immediate(expected: String, bytes: &[u8]) {
        assert!(bytes.len() >= 1);
        println!("Checking\n:{expected}");

        let rules = DecodeRules::get(&bytes[0])
            .expect("Should be a defined opcode mapping");
        let mut instruction = Bytes::Uknown(bytes[0]); // Set to error case.

        for rule in rules { // We dont know which rule will decode into an instruction
            let length = rule.len();
            println!("rule reported byte length: {length}");
            let prospective_bytes = bytes.get(0..length)
                .expect("Test should have enough bytes for decoding instruction");
            instruction = Bytes::from(prospective_bytes, rule.clone());
            println!("{instruction:?}.length() = {}", instruction.length());
            if instruction.decoded_successfully() { break; }
        }

        let mut output = Output::new(10);
        output.add(instruction).expect("This manually decoded instruction should be valid");
        assert_eq!(output.to_string(), expected);
    }

    #[test]
    fn immediate() {
        let mapping: Vec<(&str, &[u8])> = vec![
            ("00000000: 68 DD CC BB AA     push 0xAABBCCDD",     &[0x68, 0xDD, 0xCC, 0xBB, 0xAA]),
            ("00000000: CD 03     int 0x03",                     &[0xCD, 0x03                  ]),
            ("00000000: 05 DD CC BB AA     add eax, 0xAABBCCDD", &[0x05, 0xDD, 0xCC, 0xBB, 0xAA]),
        ];
        mapping.iter()
            .for_each(|(s,b)| { check_immediate(s.to_string(),b); });
    }

    //#[test]
    //fn displacement() {
        //let mapping: Vec<(&str, &[u8])> = vec![
            //("jnz 0x80",        &[0x75, 0x6E                  ]),
            //("call 0xAABBCCDD", &[0xE8, 0xD8, 0xBC, 0xBB, 0xAA]),
        //];
        //check_instructions(mapping);
    //}

    //#[test]
    //fn opcode_and_immediate() {
        //let mapping: Vec<(&str, &[u8])> = vec![
            //("mov ebx, 0xAABBCCDD", &[0xBB, 0xDD, 0xCC, 0xBB, 0xAA]),
        //];
        //check_instructions(mapping);
    //}

    //#[test]
    //fn m_rm() {
        //let mapping: Vec<(&str, &[u8])> = vec![
            //("push ecx",
             //&[0xFF, 0xF1]),

            //("push [ 0xAABBCCDD ]",
             //&[0xFF, 0x35, 0xDD, 0xCC, 0xBB, 0xAA]),

            //("dec [ ebx ]",
             //&[0xFF, 0x0B]),

            //("inc [ eax + 0x30 ]",
             //&[0xFF, 0x40, 0x30]),

            //("push [ esi + 0xAABBCCDD ]",
             //&[0xFF, 0xB6, 0xDD, 0xCC, 0xBB, 0xAA]),

            //("inc [ eax + ebx + 0xAABBCCDD ]",
             //&[0xFF, 0x84, 0x03, 0xDD, 0xCC, 0xBB, 0xAA]),

            //("push [ esi + 0xAABBCCDD ]",
             //&[0xFF, 0x34, 0x35, 0xDD, 0xCC, 0xBB, 0xAA]),

            //("call [ esp ]",
             //&[0xFF, 0x14, 0x24                        ]),
        //];
        //check_instructions(mapping);
    //}

    //#[test]
    //fn m1_rm_and_one() {
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
