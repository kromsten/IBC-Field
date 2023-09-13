use cosmwasm_std::{Response, Addr, Coin, CosmosMsg, BankMsg, Deps, DepsMut};

use crate::{error::ContractError, state::{ADMIN, AppStatus, APP_STATUS}};


pub fn set_app_status(
    deps: DepsMut,
    sender: Addr,
    status: AppStatus,
) -> Result<Response, ContractError> {
    if sender != ADMIN.load(deps.storage)? {
        return Err(ContractError::Unauthorized {});
    }
    APP_STATUS.save(deps.storage, &status)?;
    Ok(Response::default())
}



pub fn forwards_funds(
    deps: Deps,
    sender: Addr,
    to_address: String,
    amount: Vec<Coin>,
) -> Result<Response, ContractError> {
    if sender != ADMIN.load(deps.storage)? {
        return Err(ContractError::Unauthorized {});
    }
    Ok(Response::default().add_message(CosmosMsg::Bank(BankMsg::Send { 
        to_address, amount 
    })))
}