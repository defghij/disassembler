
mod instruction;
mod decode;
mod opcodes;
mod tests;
mod input;
mod output;


fn main() {

    let result = input::get_bytes();
    if result.is_err() {
        eprintln!("Unable to acquire bytes from file: {:?}", result.err());
        return;
    }
    let _bytes = result.ok().expect("Error should have been handled above");

    println!("Hello, world!");
}
