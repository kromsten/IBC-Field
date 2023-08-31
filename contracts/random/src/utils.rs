use cosmwasm_std::{Deps, Env, StdResult};
use secret_toolkit::permit::Permit;

use crate::state::{PERMITS_KEY, Powerup};

pub fn address_from_permit(deps: Deps, env: &Env, permit: &Permit) -> StdResult<String> {
    secret_toolkit::permit::validate(
        deps,
        PERMITS_KEY,
        &permit,
        env.contract.address.to_string(),
        None,
    )
}


pub fn is_powerup_list_unique(powerup_list: &Vec<Powerup>) -> bool {
    if powerup_list.is_empty() { return true };
    powerup_list
    .iter()
    .all(
        |current| 
            powerup_list
            .iter()
            .filter(|pup| pup == &current)
            .count() <= 1
    )
}

pub fn is_powerup_included(powerup_list: &Vec<Powerup>, powerup: Powerup) -> bool {
    if powerup_list.is_empty() { return false };

    powerup_list
    .iter()
    .any(|pup| pup == &powerup)
}

