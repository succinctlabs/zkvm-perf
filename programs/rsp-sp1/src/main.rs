#![no_main]

use rsp_client_executor::{io::ClientExecutorInput, ClientExecutor, EthereumVariant};

sp1_zkvm::entrypoint!(main);

pub fn main() {
    // Read the input.
    let input: Vec<u8> = sp1_zkvm::io::read_vec();
    let input = bincode::deserialize::<ClientExecutorInput>(&input).unwrap();

    // Execute the block.
    let executor = ClientExecutor;
    let header = executor.execute::<EthereumVariant>(input).expect("failed to execute client");
    let block_hash = header.hash_slow();

    println!("block_hash: {:?}", block_hash);
}