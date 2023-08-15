use cosmwasm_std::{entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, StdError};
use secret_toolkit::permit::Permit;

use crate::{msg::{ExecuteMsg, InstantiateMsg, QueryMsg}, state::{RANDOM_NUMBERS, PERMITS_KEY}};

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
pub fn execute(deps: DepsMut, env: Env, _info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
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

fn try_saving_random_number(deps: DepsMut, env: Env, permit: Permit) -> StdResult<Response> {
    let address = address_from_permit(deps.as_ref(), &env, &permit)?;
    let random = env.block.random.unwrap().0[0] % u8::MAX;
    deps.api.debug(format!("Random number is {}", random).as_str());
    RANDOM_NUMBERS.insert(deps.storage, &address, &random)?;
    Ok(Response::default())
}

fn get_saved_random_number(deps: Deps, env: Env, permit: Permit) -> StdResult<u8> {
    let address = address_from_permit(deps, &env, &permit)?;
    let number = RANDOM_NUMBERS.get(deps.storage, &address);
    if number.is_none() {
        return Err(StdError::generic_err("No random number was saved for this address"));
    }
    Ok(number.unwrap())
}

fn address_from_permit(deps: Deps, env: &Env, permit: &Permit) -> StdResult<String> {
    secret_toolkit::permit::validate(
        deps,
        PERMITS_KEY,
        &permit,
        env.contract.address.to_string(),
        None,
    )
}