use std::fmt;
use cosmwasm_schema::cw_serde;
use secret_toolkit::storage::iter_options::WithIter;
use secret_toolkit::storage::{KeymapBuilder, Keymap, WithoutIter, Item};
use secret_toolkit::serialization::{Bincode2, Json};


#[cw_serde]
pub struct Config {
    pub win_threshold: u16,
    pub cell_cooldown: u64,
    pub user_cooldown: u64,
}


#[cw_serde]
pub enum Powerup {
    Clover,
    Shovel,
    Fertilizer
}

impl fmt::Display for Powerup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Powerup::Clover => write!(f, "Clover"),
            Powerup::Shovel => write!(f, "Shovel"),
            Powerup::Fertilizer => write!(f, "Fertilizer"),
        }
    }
}

#[cw_serde]
pub struct  CellState {
    pub random: u8,
    pub open_at: u64
}

#[cw_serde]
pub struct  ChainAmount {
    pub to_win: u128,
    pub to_open: u128,
    pub power_ups: Vec<(Powerup, u128)>
}



#[repr(u8)]
enum Keys {
    Permits = b's',
    RandomNumbers = b'r',
    Config = b'g',
    Powerups = b'p',
    UserCooldowns = b'd',
    Cells = b'c',
    Channels = b'h',
    FieldSize = b'f',
    ChainAmounts = b'a'
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


pub static CHANNELS: Keymap<String, String, Json, WithoutIter> =
            KeymapBuilder::new(Keys::Channels.as_bytes()).without_iter().build();


pub static CONFIG: Item<Config, Bincode2> = Item::new(Keys::Config.as_bytes());

pub static FIELD_SIZE: Item<u8> = Item::new(Keys::FieldSize.as_bytes());

pub static USER_POWERUPS: Keymap<Powerup, u8, Json, WithIter> =
            KeymapBuilder::new(Keys::Powerups.as_bytes()).build();


pub static USER_COOLDOWNS: Keymap<String, u64, Bincode2, WithoutIter> =
    KeymapBuilder::new(Keys::UserCooldowns.as_bytes()).without_iter().build();


pub static CHAIN_AMOUNTS: Keymap<String, ChainAmount, Bincode2, WithoutIter> =
    KeymapBuilder::new(Keys::ChainAmounts.as_bytes()).without_iter().build();


pub static CELLS: Keymap<u8, CellState, Bincode2, WithoutIter> =
            KeymapBuilder::new(Keys::Cells.as_bytes()).without_iter().build();