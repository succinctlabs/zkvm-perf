#![no_main]

use rsp_client_executor::{io::ClientExecutorInput, ClientExecutor, EthereumVariant};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // Read the input.
    let input: Vec<u8> = risc0_zkvm::guest::env::read();
    let input = bincode::deserialize::<ClientExecutorInput>(&input).unwrap();

    // Execute the block.
    let executor = ClientExecutor;
    let header = executor.execute::<EthereumVariant>(input).expect("failed to execute client");
    let block_hash = header.hash_slow();
    println!("block_hash: {:?}", block_hash);
}