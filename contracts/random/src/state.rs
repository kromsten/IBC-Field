use cosmwasm_schema::cw_serde;

use cosmwasm_std::Addr;
use secret_toolkit::storage::{KeymapBuilder, Keymap, WithoutIter, Item};
use secret_toolkit::serialization::{Bincode2, Json};


#[cw_serde]
pub struct Config {
    pub win_threshold: u16,
    pub win_amount: u128,
    pub cell_cooldown: u64,
    pub user_cooldown: u64

}


#[cw_serde]
pub enum Powerup {
    Clover,
    Shovel,
    Fertilizer
}


#[cw_serde]
pub struct  CellState {
    pub random: u8,
    pub open_at: u64
}


#[repr(u8)]
enum Keys {
    Permits = b's',
    RandomNumbers = b'r',
    Config = b'g',
    Powerups = b'p',
    UserCooldowns = b'd',
    Cells = b'c'
}

impl Keys {
    const fn as_bytes(&self) -> &[u8] {
        let array_ref = unsafe {
            std::mem::transmute::<_, &[u8; 1]>(self) 
        };
        array_ref
    }

    const fn as_str(&self) -> &str {
        let array_ref = self.as_bytes();
        match core::str::from_utf8(array_ref) {
            Ok(a) => a,
            Err(_) => panic!("Non utf-8 key")
        }
    }
}


pub static PERMITS_KEY: &str = Keys::Permits.as_str();

pub static RANDOM_NUMBERS: Keymap<String, u8, Bincode2, WithoutIter> =
            KeymapBuilder::new(Keys::RandomNumbers.as_bytes()).without_iter().build();


pub static CONFIG: Item<Config, Bincode2> = Item::new(Keys::Config.as_bytes());

pub static POWERUPS: Keymap<Addr, (Powerup, u8), Json, WithoutIter> =
            KeymapBuilder::new(Keys::Powerups.as_bytes()).without_iter().build();

pub static USER_COOLDOWNS: Keymap<Addr, u64, Json, WithoutIter> =
    KeymapBuilder::new(Keys::UserCooldowns.as_bytes()).without_iter().build();


pub static CELLS: Keymap<u8, CellState, Bincode2, WithoutIter> =
            KeymapBuilder::new(Keys::Cells.as_bytes()).without_iter().build();