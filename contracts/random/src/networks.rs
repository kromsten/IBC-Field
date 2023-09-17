use cosmwasm_std::{Deps, StdResult};

use crate::state::{NETWORK_CONFIGS, NetworkConfig};



pub fn get_network_config_by_chain_id(deps: Deps, chain_id: &String) -> StdResult<Option<NetworkConfig>> {
    let configs = NETWORK_CONFIGS
        .iter(deps.storage).unwrap()
        .map(|res| res.unwrap().1)
        .find(|net| net.chain_id == *chain_id);

    Ok(configs)
}



pub fn get_network_config(deps: Deps, token: String) -> Option<NetworkConfig> {
    NETWORK_CONFIGS.get(deps.storage, &token)
}


pub fn get_all_network_configs(deps: Deps) -> StdResult<Vec<(String, NetworkConfig)>> {
    let configs: Vec<(String, NetworkConfig)> = NETWORK_CONFIGS
        .iter(deps.storage)?
        .map(|item| item.unwrap())
        .collect();
    Ok(configs)
}