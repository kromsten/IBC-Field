use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;
use secret_toolkit::permit::Permit;
use crate::state::{NetworkConfig, Powerup, AppStatus, Config};

#[cw_serde]
pub struct InstantiateMsg {
    pub field_size: Option<u8>,
    pub cell_cooldown: Option<u64>,
    pub user_cooldown: Option<u64>,
    pub win_threshold: Option<u16>,
    pub network_configs: Vec<(String, NetworkConfig)>
}

#[cw_serde]
pub enum ExecuteMsg {

    OpenCell {
        cell_id: u8,
        permit: Permit,
        powerups: Vec<Powerup>,
        power_up_autopay: Option<bool>
    },

    BuyPowerups {
        powerups: Vec<Powerup>,
        permit: Permit,
    },

    RedeemReward {
        permit: Permit,
    },

    SetAppStatus {
        status: AppStatus,
    },

    ForwardsFunds {
        to_address: String,
        amount: Vec<Coin>,
    }
}


#[cw_serde]
pub enum IBCLifecycleComplete {
    #[serde(rename = "ibc_ack")]
    IBCAck {
        /// The source channel (secret side) of the IBC packet
        channel: String,
        /// The sequence number that the packet was sent with
        sequence: u64,
        /// String encoded version of the ack as seen by OnAcknowledgementPacket(..)
        ack: String,
        /// Weather an ack is a success of failure according to the transfer spec
        success: bool,
    },
    #[serde(rename = "ibc_timeout")]
    IBCTimeout {
        /// The source channel (secret side) of the IBC packet
        channel: String,
        /// The sequence number that the packet was sent with
        sequence: u64,
    },
}


#[cw_serde]
pub enum QueryMsg {

    GetField {},

    GetMyPowerups {
        permit: Permit
    },

    AllNetworkConfigs {},
    
    NetworkConfig {
        denom: String
    },

    Main {
        permit: Option<Permit>
    },

    Config {}
}

#[cw_serde]
pub enum SudoMsg {
    #[serde(rename = "ibc_lifecycle_complete")]
    IBCLifecycleComplete(IBCLifecycleComplete),
}


#[cw_serde]
pub struct TransferIBCRewardsMsg {
    pub channel: String,
    pub remote_address: String,
    pub timeout: u64,
}


#[cw_serde]
pub struct OpenCellResponse {}


#[cw_serde]
pub struct CellResInfo {
    pub open_at: u64,
}



#[cw_serde]
pub struct GetFieldResponse {
    pub cells: Vec::<CellResInfo>
}


#[cw_serde]
pub struct MainPageResponse {
    pub config: Config,
    pub cells: Vec::<CellResInfo>,
    pub powerups: Option<Vec::<(Powerup, u8)>>,
    pub network_configs: Vec::<(String, NetworkConfig)>
}
