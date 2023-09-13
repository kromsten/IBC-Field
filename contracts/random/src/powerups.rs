use cosmwasm_std::{Deps, Env, StdResult, DepsMut, Response, Coin};
use secret_toolkit::permit::Permit;

use crate::{utils::address_from_permit, state::{Powerup, USER_POWERUPS, NETWORK_CONFIGS}, error::ContractError};

pub fn get_user_powerups(deps: Deps, env: Env, permit: Permit) -> StdResult<Vec<(Powerup, u8)>> { 
    let address = address_from_permit(deps, &env, &permit)?;
    let powerups: Vec<(Powerup, u8)> = USER_POWERUPS.add_suffix(address.as_bytes())
        .iter(deps.storage)?
        .map(|res| res.unwrap())
        .collect();
    Ok(powerups)
}


pub fn try_buying_powerups(
    deps: DepsMut,
    sender: String,
    powerups: Vec<Powerup>,
    mut funds: Vec<Coin>
) -> Result<Response, ContractError> {
    if powerups.len() == 0 {
        return Err(ContractError::EmptyPowerupList{});
    }
    if funds.len() == 0 {
        return Err(ContractError::NotPaidAtAll {});
    };
    let coin = funds.pop().unwrap();
    if funds.len() > 0 {
        return Err(ContractError::TooManyDenoms{ });
    };
    
    let config = NETWORK_CONFIGS.get(deps.storage, &coin.denom).unwrap();
    
    let mut to_pay = 0;

    let user_owned_powerups = USER_POWERUPS.add_suffix(sender.as_bytes());

    if powerups.len() > 0 {
        for powerup in powerups.iter() {
            let count = user_owned_powerups.get(deps.storage, &powerup).unwrap_or(0);
            let price = config.power_ups.iter().find(|(pup, _)| pup == powerup).unwrap().1;
            
            to_pay += price;

            user_owned_powerups.insert(deps.storage, &powerup, &(count + 1))?;
        }
    }

    if coin.amount.u128() < to_pay {
        return Err(ContractError::NotPaidEnough(to_pay, coin.amount.u128()));
    }


    Ok(Response::default())
}

