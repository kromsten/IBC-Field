use cosmwasm_std::{CosmosMsg, Attribute, BankMsg, coins, Env, IbcMsg, coin, IbcTimeout, DepsMut};
use crate::{error::ContractError, state::{NETWORK_CONFIGS, PENDIND_IBC_REWARDS}};

const SCRT_DENOM : &str = "uscrt";


pub fn reward(
    deps: DepsMut,
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

        let coin = coin(amount, denom);

        PENDIND_IBC_REWARDS.insert(deps.storage, &to_address, &coin)?;

        IbcMsg::Transfer {
            channel_id: denom_config.channel_id.unwrap(),
            to_address,
            amount: coin,
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
