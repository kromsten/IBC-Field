use cosmwasm_std::Response;
use crate::error::ContractError;


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

