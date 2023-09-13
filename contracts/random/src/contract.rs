use cosmwasm_std::{
    entry_point, 
    to_binary, 
    Binary, 
    Deps, 
    DepsMut, 
    Env, 
    MessageInfo, 
    Response, 
    StdResult
};

use rand_chacha::{
    ChaChaRng, 
    rand_core::{
        SeedableRng, CryptoRngCore
    }
};


use crate::{
    msg::{ExecuteMsg, QueryMsg, IBCLifecycleComplete, SudoMsg, InstantiateMsg}, 
    random::{try_saving_random_number, get_saved_random_number, randomness_seed}, error::ContractError,
    ibc::{ibc_transfer_incoming, ibc_lifecycle_complete, ibc_timeout}, 
    state::{CellState, CELLS, Config, CONFIG, FIELD_SIZE, CHAIN_AMOUNTS}, 
    field::{valid_field_size, try_opening_cell, try_buying_powerups, get_user_powerups}, utils::{address_from_permit, is_powerup_list_unique}
};


pub const ONE_DAY : u64 = 24 * 3600;


#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    deps.api
        .debug(format!("Contract was initialized by {}", info.sender).as_str());

    // field size
    let field_size = msg.field_size.unwrap_or(64);
    if !valid_field_size(field_size) {
        return Err(ContractError::InvalidFieldSize {});
    }
    
    FIELD_SIZE.save(deps.storage, &field_size)?;

    // win amounts
    for (denom, amounts) in msg.chain_amounts.iter() {
        let powerup_list = amounts.power_ups
                .iter().map(|(powerup, _)| powerup.clone()).collect::<Vec<_>>();

        if amounts.power_ups.len() != 3 || !is_powerup_list_unique(&powerup_list) {
            return Err(ContractError::InvalidPowerupAmounts {});
        }
        CHAIN_AMOUNTS.insert(deps.storage, &denom, &amounts).unwrap();
    }

    // config
    let cell_cooldown = msg.cell_cooldown.unwrap_or(2*ONE_DAY);
    let user_cooldown = msg.user_cooldown.unwrap_or(ONE_DAY);
    // default around 4%
    let win_threshold = msg.win_threshold.unwrap_or(u8::MAX as u16 * 2 - 20u16);


    CONFIG.save(deps.storage, &Config { 
        win_threshold, 
        cell_cooldown, 
        user_cooldown,
    })?;

    let mut ring = ChaChaRng::from_seed(
        randomness_seed(&env.block, info.sender.as_str())
    );
    let generator = ring.as_rngcore();
    for i in 1..(field_size+1) {
        CELLS.insert(deps.storage, &i, &CellState {
            random: (generator.next_u32() % u8::MAX as u32) as u8,
            open_at: env.block.time.seconds()
        })?
    }

    Ok(Response::default())
}


#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateMyRandomNumber { 
            permit 
        } => try_saving_random_number(deps, env, permit),

        ExecuteMsg::OpenCell { 
            permit,
            cell_id,
            powerups,
            power_up_autopay
        } => {

            let sender = address_from_permit(deps.as_ref(), &env, &permit)?;
                
            try_opening_cell(
                deps, 
                env, 
                sender,
                cell_id, 
                powerups, 
                power_up_autopay,
                info.funds
            )
        },

        ExecuteMsg::BuyPowerups { 
            permit,
            powerups,
        } => {

            let sender = address_from_permit(deps.as_ref(), &env, &permit)?;
                
            try_buying_powerups(
                deps, 
                sender,
                powerups,
                info.funds
            )
        },

        ExecuteMsg::TempTest { } => {
            deps.api.debug("Temp Test");
            deps.api.debug(format!("Message from {}", info.sender).as_str());
            Ok(Response::default())
        },
        
        ExecuteMsg::IBCTransfer {
            channel_id,
            to_address,
            amount,
            timeout_sec_from_now,
        } => ibc_transfer_incoming(env, channel_id, to_address, amount, timeout_sec_from_now),

        ExecuteMsg::IBCLifecycleComplete(IBCLifecycleComplete::IBCAck {
            channel,
            sequence,
            ack,
            success,
        }) => ibc_lifecycle_complete(channel, sequence, ack, success),

        ExecuteMsg::IBCLifecycleComplete(IBCLifecycleComplete::IBCTimeout { 
            channel, 
            sequence 
        }) => ibc_timeout(channel, sequence)
    }
}



#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMyRandomNumber { permit } => to_binary(&get_saved_random_number(deps, env, permit)?),
        QueryMsg::GetMyPowerups { permit } => to_binary(&get_user_powerups(deps, env, permit)?),

    }
}


#[entry_point]
pub fn sudo(_deps: DepsMut, _env: Env, msg: SudoMsg) -> StdResult<Response> {
    match msg {
        SudoMsg::IBCLifecycleComplete(IBCLifecycleComplete::IBCAck {
            channel: _,
            sequence: _,
            ack: _,
            success: _,
        }) => todo!(),
        SudoMsg::IBCLifecycleComplete(IBCLifecycleComplete::IBCTimeout {
            channel: _,
            sequence: _,
        }) => todo!(),
    }
}