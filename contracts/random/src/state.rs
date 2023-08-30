use cosmwasm_schema::cw_serde;

use cosmwasm_std::{Addr, Timestamp, BlockInfo};
use secret_toolkit::storage::{KeymapBuilder, Keymap, WithoutIter};
use secret_toolkit::serialization::{Bincode2, Json};


pub static PERMITS_KEY: &str = "permits";

pub static RANDOM_NUMBERS: Keymap<String, u8, Bincode2, WithoutIter> =
            KeymapBuilder::new(b"random").without_iter().build();



#[cw_serde]
pub enum Powerup {
    Clover,
    Shovel,
    Fertilizer
}

#[cw_serde]
pub struct  CellState {
    probability: u8,
    last_open: Timestamp
}

impl CellState {
    pub fn open(&mut self, block: BlockInfo) {
        self.probability =  block.random.unwrap().0[0] % u8::MAX;
        self.last_open = block.time;
    }
}



pub static POWERUPS: Keymap<Addr, (Powerup, u8), Json, WithoutIter> =
            KeymapBuilder::new(b"powerups").without_iter().build();


pub static CELLS: Keymap<String, Addr, Bincode2, WithoutIter> =
            KeymapBuilder::new(b"cells").without_iter().build();