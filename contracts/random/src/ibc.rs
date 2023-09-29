use cosmwasm_std::{Response, Env, Coin, Uint64, CosmosMsg, IbcMsg, IbcTimeout, DepsMut};
use crate::{error::ContractError, state::{PENDIND_IBC_REWARDS, LAST_REWARD_RECIPIENT}};




pub fn ibc_transfer_accept(
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
    deps: DepsMut,
    channel : String,
    sequence : u64 ,
    ack : String,
    success: bool
) -> Result<Response, ContractError> {

    let last = LAST_REWARD_RECIPIENT.load(deps.storage);

    if let Ok(last) = last {
        // successful payment
        PENDIND_IBC_REWARDS.remove(deps.storage, &last)?;
    } else {
        // failed payment
    }

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

