use cosmwasm_std::{DepsMut, Env, StdResult, Response, Deps, StdError};
use secret_toolkit::permit::Permit;

use crate::{utils::address_from_permit, state::RANDOM_NUMBERS};

pub fn try_saving_random_number(deps: DepsMut, env: Env, permit: Permit) -> StdResult<Response> {
    let address = address_from_permit(deps.as_ref(), &env, &permit)?;
    let random = env.block.random.unwrap().0[0] % u8::MAX;
    deps.api.debug(format!("Random number is {}", random).as_str());
    RANDOM_NUMBERS.insert(deps.storage, &address, &random)?;
    Ok(Response::default())
}

pub fn get_saved_random_number(deps: Deps, env: Env, permit: Permit) -> StdResult<u8> { 
    let address = address_from_permit(deps, &env, &permit)?;
    let number = RANDOM_NUMBERS.get(deps.storage, &address);
    if number.is_none() {
        return Err(StdError::generic_err("No random number was saved for this address"));
    }
    Ok(number.unwrap())
}
