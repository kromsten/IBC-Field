use cosmwasm_std::{CosmosMsg, Attribute, BankMsg, Deps, Addr, coins};
use crate::error::ContractError;

const SCRT_DENOM : &str = "uscrt";
// const AKT_DENOM : &str = "uakt";


pub fn reward(
    deps: Deps,
    sender: Addr,
    amount: u128,
    msgs: &mut Vec<CosmosMsg>,
    attributes: &mut Vec<Attribute>,
) -> Result<(), ContractError> {

    let msg: CosmosMsg = if sender.as_str().starts_with("secret") {
        reward_scrt(deps, sender, amount)?.into()
    } else {
        reward_remote(deps, sender, amount)?.into()
    };

    // TODO: Add attributes
    attributes.push(Attribute::new("action", "reward"));

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
    _deps: Deps,
    _sender: Addr,
    _amount: u128,
) -> Result<BankMsg, ContractError> {
    todo!("Implement this")
}