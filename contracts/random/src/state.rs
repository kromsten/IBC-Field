use cosmwasm_schema::cw_serde;

use cosmwasm_std::Addr;
use secret_toolkit::storage::{KeymapBuilder, Keymap, WithoutIter};
use secret_toolkit::serialization::{Bincode2};


pub static PERMITS_KEY: &str = "permits";

pub static RANDOM_NUMBERS: Keymap<String, u8, Bincode2, WithoutIter> =
            KeymapBuilder::new(b"random").without_iter().build();