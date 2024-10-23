mod utils;

use bincode::Options;
use clap::Parser;
use crate::utils::init::ValidaRethInputInitializer;
use reth_valida::primitives::ValidaRethInput;
use std::fs::{File, OpenOptions};
use std::io::Write;

/// The CLI arguments for the Valida Reth program.
#[derive(Parser, Debug)]
pub struct ValidaRethArgs {
    #[arg(short, long)]
    rpc_url: String,

    #[arg(short, long)]
    block_number: u64,

    #[arg(short, long)]
    use_cache: bool,
}

#[tokio::main]
async fn main() {
    // Parse arguments.
    let args = ValidaRethArgs::parse();

    // Get input.
    let input: ValidaRethInput = if !args.use_cache {
        let input = ValidaRethInput::initialize(&args.rpc_url, args.block_number)
            .await
            .unwrap();
        let mut file =
            File::create(format!("{}.bin", args.block_number)).expect("unable to open file");
        bincode::options()
            .with_little_endian()
            .serialize_into(&mut file, &input).expect("unable to serialize input");
        input
    } else {
        let file = File::open(format!("{}.bin", args.block_number)).expect("unable to open file");
        bincode::options()
            .with_little_endian()
            .deserialize_from(file).expect("unable to deserialize input")
    };

    let bytes = bincode::options()
        .with_little_endian()
        .serialize(&input).unwrap();
    let len = bytes.len().to_string();
    
    // Create a file that appends when writing.
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("input.bin")
        .expect("Failed to open file");
    
    // Write the length of the input to the file. (First input to Valida's read)
    file.write_all(len.as_bytes()).expect("Failed to write length to file");
    file.write(&['\n' as u8]).expect("Failed to write newline to file");

    // Append the input directly to a file using bincode
    bincode::options()
        .with_little_endian()
        .serialize_into(&mut file, &input).expect("Failed to write input to file");
    
    println!("Input has been written to input.bin");
}
