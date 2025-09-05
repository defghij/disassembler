// This contains tests from the course 
//#[cfg(test)]
//pub mod compendium {
    //use crate::decode::DecodeRule;

    //#[inline(always)]
    //fn check_instructions(mapping: Vec<(&str, &[u8])>) {
        //mapping.iter().for_each(|(s,b)|{
            //let int3_s: DecodeRule = s
                //.to_string()
                //.try_into()
                //.expect("Should be a defined instruction");
            //let int3_b: DecodeRule = b
                //.as_ref()
                //.try_into()
                //.expect("Should be a defined instruction");
            //assert_eq!(int3_s, int3_b);
        //});
    //}

    //#[test]
    //fn zero() {
        //let mapping: Vec<(&str, &[u8])> = vec![
            //("int3", &[0xCC]),
            //("cdp", &[0x99]),
            //("ret", &[0xC3]),
            //("ret", &[0xCB]),
        //];
        //check_instructions(mapping);
    //}

    //#[test]
    //fn opcode() {
        //let mapping: Vec<(&str, &[u8])> = vec![
            //("dec ebx",  &[0x4B]),
            //("inc eax",  &[0x40]),
            //("dec eax",  &[0x48]),
            //("push ecx", &[0x51]),
            //("pop edi",  &[0x5F]),
        //];
        //check_instructions(mapping);
    //}

    //#[test]
    //fn immediate() {
        //let mapping: Vec<(&str, &[u8])> = vec![
            //("push 0xAABBCCDD",     &[0x68, 0xDD, 0xCC, 0xBB, 0xAA]),
            //("int 3",               &[0xCD, 0x03                  ]),
            //("add eax, 0xAABBCCDD", &[0x05, 0xDD, 0xCC, 0xBB, 0xAA]),
        //];
        //check_instructions(mapping);
    //}

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
//}
