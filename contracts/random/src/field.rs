use cosmwasm_std::{Response, Env, DepsMut, Addr, CosmosMsg, BankMsg, coins, Deps, Attribute};
use std::cmp;

use crate::{state::{CELLS, Powerup, CONFIG, USER_COOLDOWNS}, error::ContractError, utils::{is_powerup_list_unique, is_powerup_included}, random::randomness_using_address};

const SCRT_DENOM : &str = "uscrt";
const AKT_DEONM : &str = "uakt";

pub fn try_opening_cell(
    deps: DepsMut,
    env: Env,
    sender: Addr,
    cell_id: u8,
    powerups: Vec<Powerup>
) -> Result<Response, ContractError> {
    let cell = CELLS.get(deps.storage, &cell_id);
    if cell.is_none() {
        return Err(ContractError::NotFound(cell_id).into());
    }

    if !is_powerup_list_unique(&powerups) {
        return Err(ContractError::NotUnique {});
    }

    let mut cell = cell.unwrap();

    let config = CONFIG.load(deps.storage)?;

    if cell.open_at > env.block.time.seconds() 
        && !is_powerup_included(&powerups, Powerup::Fertilizer) {
        return Err(ContractError::CellCooldown(cell.open_at - env.block.time.seconds()));
    }

    let cooldown = USER_COOLDOWNS.get(deps.storage, &sender);
    if cooldown.is_some() {
        let cooldown = cooldown.unwrap();
        if cooldown > env.block.time.seconds() && !is_powerup_included(&powerups, Powerup::Shovel) {
            return Err(ContractError::UserCooldown(cooldown - env.block.time.seconds()));
        }
    }

    let block_random = env.block.random.as_ref().unwrap().clone().0;
    let mut user_random = randomness_using_address(sender.as_str(), block_random[1]);
    if is_powerup_included(&powerups, Powerup::Clover) {
        user_random = cmp::min(u8::MAX, user_random * 2);
    }

    let mut msgs = Vec::<CosmosMsg>::with_capacity(1);
    let mut attributes = vec![];

    if (cell.random as u16 + user_random as u16) > config.win_threshold {
        reward(deps.as_ref(), sender.clone(), config.win_amount, &mut msgs, &mut attributes)?
    }


    cell.open_at = env.block.time.seconds() + config.cell_cooldown;
    cell.random = block_random[0];

    CELLS.insert(deps.storage, &cell_id, &cell)?;
    USER_COOLDOWNS.insert(deps.storage, &sender, &(env.block.time.seconds() + config.user_cooldown))?;
    
    Ok(Response::default().add_messages(msgs).add_attributes(attributes))
}


fn reward(
    deps: Deps,
    sender: Addr,
    amount: u128,
    msgs: &mut Vec<CosmosMsg>,
    attributes: &mut Vec<Attribute>
) -> Result<(), ContractError> {

    let msg: CosmosMsg = if sender.as_str().starts_with("secret") {
        reward_scrt(deps, sender, amount)?.into()
    } else {
        reward_remote(deps, sender, amount)?.into()
    };

    // TODO: Add attributes

    msgs.push(msg);

    Ok(())
}

fn reward_scrt(
    deps: Deps,
    sender: Addr,
    amount: u128,
) -> Result<BankMsg, ContractError> {

    let response = deps.querier.query_balance(sender.clone(), SCRT_DENOM)?;
    if response.amount.u128() < amount {
        return Err(ContractError::NotEnoughFundsNative(amount, response.amount.u128()));
    }
    Ok(BankMsg::Send { 
        to_address: sender.to_string(), 
        amount: coins(amount, SCRT_DENOM)
    })
}

fn reward_remote(
    deps: Deps,
    sender: Addr,
    amount: u128,
) -> Result<BankMsg, ContractError> {
    todo!("Implement this")
}