use cosmwasm_std::{CosmosMsg, Attribute, BankMsg, Deps, coins, Env, IbcMsg, coin, IbcTimeout};
use crate::{error::ContractError, state::NETWORK_CONFIGS};

const SCRT_DENOM : &str = "uscrt";
// const AKT_DENOM : &str = "uakt";


pub fn reward(
    deps: Deps,
    env: Env,
    to_address: String,
    denom: String,
    msgs: &mut Vec<CosmosMsg>,
    attributes: &mut Vec<Attribute>,
) -> Result<(), ContractError> {

    let denom_config = NETWORK_CONFIGS.get(deps.storage, &denom).unwrap();
    let amount = denom_config.to_win;

    let response = deps.querier.query_balance(env.contract.address.clone(), denom.clone()
)?;
    if response.amount.u128() < amount {
        return Err(ContractError::NotEnoughFundsNative(amount, response.amount.u128()));
    }

    let msg: CosmosMsg = if denom == SCRT_DENOM {
        BankMsg::Send { 
            to_address: to_address.to_string(), 
            amount: coins(amount, SCRT_DENOM)
        }.into()
    } else {
        IbcMsg::Transfer {
            channel_id: denom_config.channel_id.unwrap(),
            to_address: to_address,
            amount: coin(amount, denom),
            timeout: IbcTimeout::with_timestamp(
                env.block.time.plus_seconds(5*60),
            ),
            memo: format!( "{{\"ibc_callback\":\"{}\"}}", env.contract.address.to_string()),
        }.into()
    };

    // TODO: Add attributes
    attributes.push(Attribute::new("action", "reward"));
    
    msgs.push(msg);

    Ok(())
}
