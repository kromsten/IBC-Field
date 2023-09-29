use cosmwasm_std::{Deps, Env, StdResult};
use secret_toolkit::permit::Permit;

use crate::{state::{PERMITS_KEY, Powerup}, networks::get_network_config_by_chain_id};



pub fn address_from_permit(
    deps: Deps, 
    env: &Env, 
    permit: &Permit,
) -> StdResult<String> {

    deps.api.debug("ADDRESS FROM PERMIT:");
    let config = get_network_config_by_chain_id(deps, &permit.params.chain_id);

    deps.api.debug("PERMIT VALIDATION:");
    deps.api.debug(format!("network config: {:?}", config).as_str());
    deps.api.debug(format!("permit: {:?}", permit).as_str());

    secret_toolkit::permit::validate(
        deps,
        PERMITS_KEY,
        &permit,
        env.contract.address.to_string(),
        None,
    )

}


pub fn is_chain_id_list_unique(chain_ids: &Vec<String>) -> bool {
    if chain_ids.is_empty() { return true };
    chain_ids
    .iter()
    .all(
        |current| 
        chain_ids
            .iter()
            .filter(|pup| pup == &current)
            .count() <= 1
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

pub fn is_powerup_included(powerup_list: &Vec<Powerup>, powerup: &Powerup) -> bool {
    if powerup_list.is_empty() { return false };
    powerup_list
                .iter()
                .any(|pup| pup == powerup)
}

