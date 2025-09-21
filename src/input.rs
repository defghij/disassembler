use std::path::PathBuf;
use clap::{
    Arg, 
    ArgMatches,
    Command,
};


fn path_from_cli() -> PathBuf {
    let args: ArgMatches = Command::new("disassemble")
        .about("Utility to disassemble a subset of x86")
        .version("0.1.0")
        .author("cnorri17@jh.edu")
        .arg(
            Arg::new("file")
                .long("input")
                .short('i')
                .value_parser(clap::value_parser!(PathBuf))
                .required(true)
                .help("Binary file to disassemble")
        ).get_matches();

    let path: PathBuf = args.get_one::<PathBuf>("file")
        .expect("File should be a required argument that is validated by Clap")
        .clone();
    path
}

pub fn read_bytes_from_cli() -> Result<Vec<u8>, std::io::Error> {
    let path = path_from_cli();
    get_bytes(path)
}

pub fn get_bytes(path: PathBuf) -> Result<Vec<u8>, std::io::Error> {
    let bytes = std::fs::read(&path)?;

    println!("Read {} bytes from {}", 
        bytes.len(), 
        path.to_str().expect("File path should have been validated by Clap"));

    Ok(bytes)
}


