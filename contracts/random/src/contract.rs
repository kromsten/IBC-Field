use cosmwasm_std::{
    entry_point, 
    to_binary, 
    Binary, 
    Deps, 
    DepsMut, 
    Env, 
    MessageInfo, 
    Response, 
    StdResult
};
use schemars::_serde_json::de;

use crate::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg, IBCLifecycleComplete, SudoMsg}, 
    random::{try_saving_random_number, get_saved_random_number}, error::ContractError,
    ibc::{ibc_transfer_incoming, ibc_lifecycle_complete, ibc_timeout}
};


#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {

    deps.api
        .debug(format!("Contract was initialized by {}", info.sender).as_str());

    for _ in 0..64 {
        let random = env.block.random.as_ref().unwrap().0[0] % u8::MAX;
        deps.api.debug(format!("Random number is {}", random).as_str());
    }


    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateMyRandomNumber { 
            permit 
        } => try_saving_random_number(deps, env, permit),

        ExecuteMsg::TempTest { } => {
            deps.api.debug("Temp Test");
            deps.api.debug(format!("Message from {}", info.sender).as_str());
            Ok(Response::default())
        },
        
        ExecuteMsg::IBCTransfer {
            channel_id,
            to_address,
            amount,
            timeout_sec_from_now,
        } => ibc_transfer_incoming(env, channel_id, to_address, amount, timeout_sec_from_now),

        ExecuteMsg::IBCLifecycleComplete(IBCLifecycleComplete::IBCAck {
            channel,
            sequence,
            ack,
            success,
        }) => ibc_lifecycle_complete(channel, sequence, ack, success),

        ExecuteMsg::IBCLifecycleComplete(IBCLifecycleComplete::IBCTimeout { 
            channel, 
            sequence 
        }) => ibc_timeout(channel, sequence)
    }
}


#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMyRandomNumber { permit } => to_binary(&get_saved_random_number(deps, env, permit)?),
    }
}



#[entry_point]
pub fn sudo(_deps: DepsMut, _env: Env, msg: SudoMsg) -> StdResult<Response> {
    match msg {
        SudoMsg::IBCLifecycleComplete(IBCLifecycleComplete::IBCAck {
            channel: _,
            sequence: _,
            ack: _,
            success: _,
        }) => todo!(),
        SudoMsg::IBCLifecycleComplete(IBCLifecycleComplete::IBCTimeout {
            channel: _,
            sequence: _,
        }) => todo!(),
    }
}