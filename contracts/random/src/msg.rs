use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Uint64, Coin};
use secret_toolkit::permit::Permit;
use crate::state::ChainAmount;

#[cw_serde]
pub struct InstantiateMsg {
    pub field_size: Option<u8>,
    pub cell_cooldown: Option<u64>,
    pub user_cooldown: Option<u64>,
    pub win_threshold: Option<u16>,
    pub chain_amounts: Vec<(String, ChainAmount)>
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateMyRandomNumber {
        permit: Permit
    },

    TempTest {},

    #[serde(rename = "ibc_transfer")]
    IBCTransfer {
        channel_id: String,
        to_address: String,
        amount: Coin,
        timeout_sec_from_now: Uint64,
    },
    #[serde(rename = "ibc_lifecycle_complete")]
    IBCLifecycleComplete(IBCLifecycleComplete),
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
    GetMyRandomNumber {
        permit: Permit
    }
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


