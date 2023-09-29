use cosmwasm_std::{BlockInfo};

pub fn randomness_seed(block: &BlockInfo, address: &str) -> [u8; 32] {
    let mut random = block.random.clone().unwrap();
    // XOR last 4 bytes of address with first 4 bytes of block random
    for (i, byte) in address.as_bytes().iter().rev().enumerate().take(4) {
        random.0[i] ^= byte;
    }
    random.to_array().unwrap()
}



pub fn randomness_using_address(address: &str, seed: u8) -> u8 {
    // xor last 4 bytes of address
    let mut result = seed;
    let bytes = address.as_bytes();

    for byte in bytes.iter().rev().take(4) {
        result ^= byte;
    }
    result
}