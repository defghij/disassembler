use opcodes::DecodeRules;
use decode::{
    DecodeRule,
    Bytes,
};
use output::Output;


mod instruction;
mod decode;
mod opcodes;
mod tests;
mod input;
mod output;

#[test]
fn zero_operands() {
    let expected = "00000000: C3     retn".to_string();
    let ret = 0xC3;
    let rules = DecodeRules::get(&ret)
        .expect("`ret` should be a defined opcode mapping");
    assert!(rules.len() == 1);
    let dc_rule: &DecodeRule = rules.get(0).expect("Should be only one element");
    assert!(dc_rule.len() == 1);

    let instruction = dc_rule.mnemonic();
    let bytes = dc_rule.op_code().bytes();
    let ret = Bytes::Decoded { bytes, instruction };


    let mut output = Output::new(1);
    output.add(ret).expect("This manually decoded instruction should be valid");
    assert_eq!(output.to_string(), expected);
}

fn register_operand_single_byte() {
    let expected = "00000000: C3     retn".to_string();
    let ret = 0xC3;
    let rules = DecodeRules::get(&ret)
        .expect("`ret` should be a defined opcode mapping");
    assert!(rules.len() == 1);
    let dc_rule: &DecodeRule = rules.get(0).expect("Should be only one element");
    assert!(dc_rule.len() == 1);

    let instruction = dc_rule.mnemonic();
    let bytes = dc_rule.op_code().bytes();
    let ret = Bytes::Decoded { bytes, instruction };


    let mut output = Output::new(1);
    output.add(ret).expect("This manually decoded instruction should be valid");
    assert_eq!(output.to_string(), expected);
}

fn main() {

    let result = input::get_bytes();
    if result.is_err() {
        eprintln!("Unable to acquire bytes from file: {:?}", result.err());
        return;
    }
    let _bytes = result.ok().expect("Error should have been handled above");

    println!("Hello, world!");
}
