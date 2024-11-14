mod utils;

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
}

#[tokio::main]
async fn main() {
    // Parse arguments.
    let args = ValidaRethArgs::parse();

    // Get input.
    let input = ValidaRethInput::initialize(&args.rpc_url, args.block_number)
        .await
        .unwrap();
    let mut file =
        File::create(format!("{}.bin", args.block_number)).expect("unable to open file");
    file.write_all(
        &serde_json::ser::to_vec(&input).expect("unable to serialize input").as_slice()
    ).expect("unable to write to file");
    
    println!("Input has been written to {}.bin", args.block_number);
}
