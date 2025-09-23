mod decode;
mod input;
mod instruction;
mod opcodes;
mod output;
mod tests;

use tracing::{error, level_filters::LevelFilter};

use output::{Disassembly, setup_tracing};

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
