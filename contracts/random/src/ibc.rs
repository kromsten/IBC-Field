use cosmwasm_std::{Response, CosmosMsg, Coin, Uint64, Env, IbcMsg, IbcTimeout};
use crate::error::ContractError;


pub fn ibc_transfer(
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
        ("ibc_lifecycle_complete.ibc_timeout.sequence",sequence.to_string() ),
    ]))
}