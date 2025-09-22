mod instruction;
mod decode;
mod opcodes;
mod tests;
mod input;
mod output;

use tracing::{level_filters::LevelFilter, error};

use output::{setup_tracing, Disassembly};

fn main() {
    setup_tracing(LevelFilter::OFF);

    let result = input::read_bytes_from_cli();
    if result.is_err() {
        error!("Unable to acquire bytes from file(s): {:?}", result.err());
        return;
    }
    let bytes = result.ok().expect("Error should have been handled above");

    let output = Disassembly::from(bytes);

    println!("{output}");
}
