use std::fmt;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;
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
pub enum AppStatus {
    Active,
    Paused,
    Finished
}



#[cw_serde]
pub struct  CellState {
    pub random: u8,
    pub open_at: u64
}


#[cw_serde]
pub struct  NetworkConfig {
    pub chain_id: String,
    pub channel_id: Option<String>,
    pub hrp: Option<String>, 
    
    pub to_win: u128,
    pub to_open: u128,
    pub power_ups: Vec<(Powerup, u128)>
}


#[repr(u8)]
enum Keys {
    Permits = b'p',
    RandomNumbers = b'r',
    Config = b'g',
    Powerups = b'u',
    UserCooldowns = b'd',
    Cells = b'c',
    Channels = b'h',
    FieldSize = b'f',
    NetworkConfigs = b'n',
    Admin = b'a',
    AppStatus = b's',
    ToReward = b't',
    LastRewardRec = b'l'
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


pub static TOKEN_TO_CHANNEL: Keymap<String, String, Json, WithoutIter> =
            KeymapBuilder::new(Keys::Channels.as_bytes()).without_iter().build();

pub static ADMIN: Item<String> = Item::new(Keys::Admin.as_bytes());
pub static APP_STATUS: Item<AppStatus> = Item::new(Keys::AppStatus.as_bytes());
pub static CONFIG: Item<Config, Bincode2> = Item::new(Keys::Config.as_bytes());
pub static FIELD_SIZE: Item<u8> = Item::new(Keys::FieldSize.as_bytes());

pub static USER_POWERUPS: Keymap<Powerup, u8, Json, WithIter> =
            KeymapBuilder::new(Keys::Powerups.as_bytes()).build();

pub static USER_COOLDOWNS: Keymap<String, u64, Bincode2, WithoutIter> =
    KeymapBuilder::new(Keys::UserCooldowns.as_bytes()).without_iter().build();

pub static NETWORK_CONFIGS: Keymap<String, NetworkConfig, Json> =
    KeymapBuilder::new(Keys::NetworkConfigs.as_bytes()).with_page_size(10).build();

pub static CELLS: Keymap<u8, CellState, Bincode2> =
            KeymapBuilder::new(Keys::Cells.as_bytes()).with_page_size(64).build();


pub static PENDIND_IBC_REWARDS: Keymap<String, Coin, Bincode2, WithoutIter> = 
            KeymapBuilder::new(Keys::ToReward.as_bytes()).without_iter().build();

pub static LAST_REWARD_RECIPIENT: Item<String> = Item::new(Keys::LastRewardRec.as_bytes());