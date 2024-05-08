#![no_main]

mod beacon;
mod proof;

#[cfg(feature = "risc0")]
extern crate sha2_risc0;

#[cfg(feature = "sp1")]
extern crate sha2_sp1;

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

use beacon::hints;
use beacon::node_from_bytes;
use beacon::prove;
use beacon::types::*;
use hex_literal::hex;
use ssz_rs::prelude::*;
use std::collections::HashMap;

/// Given a beacon block root, start slot, end slot, and eigenpod address, returns the sum of all
/// beacon partial withdrawals in [start_slot, end_slot) to the given eigenpod address.
pub fn main() {
    let beacon_block_root = node_from_bytes(hex!(
        "d00c4da1a3ad4d42bd35f128544227d19e163194569d69d54a3d14112e3c897c"
    ));
    let start_slot = 7855804;
    let end_slot = 7855807;
    let eigenpod_address =
        ExecutionAddress::try_from(hex!("e9cd1419a015dd05d47f6139f5b8e86b1e9e5cdd").to_vec())
            .unwrap();

    // Get slot number from block by proving the block header.
    let source_slot = prove::block_header(beacon_block_root).slot;

    // Load the witness data from outside of the vm.
    let (withdrawal_slots, validator_indexes) =
        hints::withdrawals_range(beacon_block_root, start_slot, end_slot, &eigenpod_address);

    // For all validator_indexes in the range, prove their withdrawable epoch so we can check
    // whether each withdrawal is partial or full.
    let validators_root = prove::validators_root(beacon_block_root);
    let mut withdrawable_epochs = HashMap::<u64, u64>::new();
    for validator_index in validator_indexes {
        println!("validator index: {}", validator_index);
        let validator = prove::validator(beacon_block_root, validators_root, validator_index);
        withdrawable_epochs.insert(validator_index, validator.withdrawable_epoch);
    }

    let mut sum = 0;

    // We only have 3 slots of withdrawal data in our test vector. In practice, users will want to prove a much larger range of withdrawals.
    // To simulate this, we loop over the withdrawal_slots 10 times to simulate more merkle proof verification to mirror realistic programs.
    for _ in 0..10 {
        for (slot, withdrawal_indexes) in &withdrawal_slots {
            println!("slot: {}", slot);
            let historical_block_root =
                prove::historical_block_root(beacon_block_root, source_slot, *slot);
            let withdrawals_root = prove::withdrawals_root(historical_block_root);
            let epoch = slot / 32;
            for index in withdrawal_indexes {
                let withdrawal = prove::withdrawal(historical_block_root, withdrawals_root, *index);

                let withdrawable_epoch = withdrawable_epochs
                    .get(&withdrawal.validator_index)
                    .unwrap();
                if epoch < *withdrawable_epoch {
                    sum += withdrawal.amount;
                }
            }
        }
    }

    println!("sum: {}", sum);
}
