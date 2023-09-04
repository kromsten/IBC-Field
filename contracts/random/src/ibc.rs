use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Response, CosmosMsg, Coin, Uint64, Env, IbcTimeout, IbcMsg, DepsMut, Uint128, to_binary, Binary};
use crate::{error::ContractError, msg::TransferIBCRewardsMsg, state::CHANNELS};


#[cw_serde]
pub struct Ics20Packet {
    pub amount: Uint128,
    pub denom: String,
    pub receiver: String,
    pub sender: String,
}

impl Ics20Packet {
    pub fn new<T: Into<String>>(amount: Uint128, denom: T, sender: &str, receiver: &str) -> Self {
        Ics20Packet {
            denom: denom.into(),
            amount,
            sender: sender.to_string(),
            receiver: receiver.to_string(),
        }
    }
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.amount.u128() > (u64::MAX as u128) {
            Err(ContractError::AmountOverflow {})
        } else {
            Ok(())
        }
    }
}

#[cw_serde]
pub enum Ics20Ack {
    Result(Binary),
    Error(String),
}


pub fn ibc_transfer_incoming(
    env: Env,
    channel_id : String,
    to_address : String ,
    amount : Coin,
    timeout_sec_from_now: Uint64
) -> Result<Response, ContractError> {
    Ok(
        Response::default().add_messages(vec![CosmosMsg::Ibc(IbcMsg::Transfer {
            channel_id,
            to_address: to_address,
            amount: amount,
            timeout: IbcTimeout::with_timestamp(
                env.block.time.plus_seconds(timeout_sec_from_now.u64()),
            ),
            memo: format!(
                "{{\"ibc_callback\":\"{}\"}}",
                env.contract.address.to_string()
            ),
        })]),
    )
}


pub fn ibc_lifecycle_complete(
    channel : String,
    sequence : u64 ,
    ack : String,
    success: bool
) -> Result<Response, ContractError> {
    Ok(Response::default().add_attributes(vec![
        ("ibc_lifecycle_complete.ibc_ack.channel", channel),
        ("ibc_lifecycle_complete.ibc_ack.sequence",sequence.to_string()),
        ("ibc_lifecycle_complete.ibc_ack.ack", ack),
        ("ibc_lifecycle_complete.ibc_ack.success", success.to_string(),),
    ]))
}

pub fn ibc_timeout(
    channel : String,
    sequence : u64 ,
) -> Result<Response, ContractError> {
    Ok(Response::default().add_attributes(vec![
        ("ibc_lifecycle_complete.ibc_timeout.channel", channel),
        ("ibc_lifecycle_complete.ibc_timeout.sequence",sequence.to_string()),
    ]))
}


pub fn send_funds_over_ibc(
    deps: DepsMut,
    env: Env,
    msg: TransferIBCRewardsMsg,
    amount: u128,
    denom: String,
) -> Result<Response, ContractError> {

    // ensure the requested channel is registered
    if !CHANNELS.contains(deps.storage, &msg.channel) {
        return Err(ContractError::NotSupportedChannel { } );
    }

    // timeout is in nanoseconds
    let timeout = env.block.time.plus_seconds(msg.timeout);

    // build ics20 packet
    let packet = Ics20Packet::new(
        Uint128::from(amount),
        denom,
        env.contract.address.as_str(),
        &msg.remote_address,
    );
    packet.validate()?;

    // prepare ibc message
    let msg = IbcMsg::SendPacket {
        channel_id: msg.channel,
        data: to_binary(&packet)?,
        timeout: timeout.into(),
    };

    // send response
    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "transfer")
        .add_attribute("sender", &packet.sender)
        .add_attribute("receiver", &packet.receiver)
        .add_attribute("denom", &packet.denom)
        .add_attribute("amount", &packet.amount.to_string());
    Ok(res)
}