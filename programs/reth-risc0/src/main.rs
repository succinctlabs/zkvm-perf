//! An implementation of a type-1, bytecompatible compatible, zkEVM written in Rust & SP1.
//!
//! The flow for the guest program is based on Zeth.
//!
//! Reference: https://github.com/risc0/zeth

#![no_main]
risc0_zkvm::guest::entry!(main);

use reth_primitives::B256;
use revm::InMemoryDB;
use sp1_reth_primitives::db::InMemoryDBHelper;
use sp1_reth_primitives::mpt::keccak;
use sp1_reth_primitives::processor::EvmProcessor;
use sp1_reth_primitives::SP1RethInput;

// Include bytes from the file with the block number.

fn main() {
    // Read the input from a file.
    let bytes = include_bytes!("../17106222.bin");
    let mut input: SP1RethInput = bincode::deserialize(bytes).expect("unable to deserialize input");

    // Initialize the database.
    let db = InMemoryDB::initialize(&mut input).unwrap();

    // Execute the block.
    let mut executor = EvmProcessor::<InMemoryDB> {
        input,
        db: Some(db),
        header: None,
    };
    executor.initialize();
    executor.execute();
    executor.finalize();

    // Print the resulting block hash.
    let hash = B256::from(keccak(alloy_rlp::encode(executor.header.unwrap())));
    println!("block hash: {}", hash);
}
