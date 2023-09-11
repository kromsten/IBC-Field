use cosmwasm_std::{DepsMut, Env, StdResult, Response, Deps, StdError};
use secret_toolkit::permit::Permit;

use crate::{utils::address_from_permit, state::RANDOM_NUMBERS, error::ContractError};


pub fn randomness_using_address(address: &str, seed: u8) -> u8 {
    // xor last 4 bytes of address
    let mut result = seed;
    let bytes = address.as_bytes();

    for byte in bytes.iter().rev().take(4) {
        result ^= byte;
    }
    result
}


pub fn try_saving_random_number(deps: DepsMut, env: Env, permit: Permit) -> Result<Response, ContractError> {
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
