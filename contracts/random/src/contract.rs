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

use crate::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg}, 
    random::{try_saving_random_number, get_saved_random_number}, error::ContractError
};


#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {

    deps.api
        .debug(format!("Contract was initialized by {}", info.sender).as_str());

    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, _info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateMyRandomNumber { permit } => try_saving_random_number(deps, env, permit)
    }
}


#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMyRandomNumber { permit } => to_binary(&get_saved_random_number(deps, env, permit)?),
    }
}
