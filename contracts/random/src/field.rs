use cosmwasm_std::{Response, Env, DepsMut, CosmosMsg, Coin, Deps, Event, Attribute, StdResult};
use rand_chacha::{ChaChaRng, rand_core::{SeedableRng, CryptoRngCore}};
use crate::{
    state::{CELLS, Powerup, CONFIG, USER_COOLDOWNS, NETWORK_CONFIGS, FIELD_SIZE, USER_POWERUPS}, 
    error::ContractError, utils::{is_powerup_list_unique, is_powerup_included}, 
    random::randomness_seed, rewards::reward, msg::{GetFieldResponse, CellResInfo}
};



pub fn valid_field_size(field_size: u8) -> bool {
    field_size >= 3 && field_size <= 64 && field_size % 2 == 0
}

pub fn valid_cell_id(deps: Deps, cell_id: u8) -> bool {
    let max = FIELD_SIZE.load(deps.storage).unwrap();
    cell_id > 0 && cell_id <= max
}


pub fn get_field_cells(deps: Deps) -> StdResult<GetFieldResponse> { 
    let cells: Vec<CellResInfo> = CELLS
        .iter(deps.storage)?
        .map(|res| CellResInfo { 
            open_at: res.unwrap().1.open_at 
        })
        .collect();

    Ok(GetFieldResponse { cells })
}

pub fn try_opening_cell(
    deps: DepsMut,
    env: Env,
    sender: String,
    cell_id: u8,
    powerups: Vec<Powerup>,
    powerup_autopay: Option<bool>,
    mut funds: Vec<Coin>
) -> Result<Response, ContractError> {

    if funds.len() == 0 {
        return Err(ContractError::NotPaidAtAll {});
    }

    if !valid_cell_id(deps.as_ref(), cell_id) {
        return Err(ContractError::InvalidCellId {});
    }

    let coin = funds.pop().unwrap();
    
    if funds.len() > 0 {
        return Err(ContractError::TooManyDenoms{ });
    };


    if !is_powerup_list_unique(&powerups) {
        return Err(ContractError::NotUnique {});
    }
    let autopay = powerup_autopay.unwrap_or(false);

    let amounts = NETWORK_CONFIGS.get(deps.storage, &coin.denom);
    if amounts.is_none() {
        return Err(ContractError::NoAmountInfo{});
    }
    let amounts = amounts.unwrap();

    let mut to_pay = amounts.to_open;

    
    if powerups.len() > 0 {
        let user_owned_powerups = USER_POWERUPS.add_suffix(sender.as_bytes());
        
        for powerup in powerups.iter() {
            let count = user_owned_powerups.get(deps.storage, &powerup).unwrap_or(0);
            if count == 0 {
                if autopay {
                    let price = amounts.power_ups.iter().find(|(pup, _)| pup == powerup).unwrap().1;
                    to_pay += price;
                } else {
                    return Err(ContractError::NoPowerups{});
                }
            } else {
                user_owned_powerups.insert(deps.storage, &powerup, &(count - 1))?;
            }
        }
    }

    if coin.amount.u128() < to_pay {
        return Err(ContractError::NotPaidEnough(amounts.to_open, coin.amount.u128()));
    }

    let cell = CELLS.get(deps.storage, &cell_id);
    if cell.is_none() {
        return Err(ContractError::NotFound(cell_id).into());
    }
    let mut cell = cell.unwrap();


    let config = CONFIG.load(deps.storage)?;

    if cell.open_at > env.block.time.seconds() 
        && !is_powerup_included(&powerups, &Powerup::Fertilizer) {
        return Err(ContractError::CellCooldown(cell.open_at - env.block.time.seconds()));
    }

    let cooldown = USER_COOLDOWNS.get(deps.storage, &sender);
    if cooldown.is_some() {
        let cooldown = cooldown.unwrap();
        if cooldown > env.block.time.seconds() && !is_powerup_included(&powerups, &Powerup::Shovel) {
            return Err(ContractError::UserCooldown(cooldown - env.block.time.seconds()));
        }
    }

    let block_random = env.block.random.as_ref().unwrap().clone().0;

    let mut ring = ChaChaRng::from_seed(randomness_seed(&env.block, sender.as_str()));
    let generator = ring.as_rngcore();


    let mut user_random = generator.next_u32() % u8::MAX as u32;
    if is_powerup_included(&powerups, &Powerup::Clover) {
        user_random *= 2;
    }

    let user_cooldown_ends_at = env.block.time.seconds() + config.user_cooldown;
    let to_be_rewarded = cell.random as u16 + user_random as u16 > config.win_threshold;
    
    let cell_old_random = cell.random;

    cell.open_at = env.block.time.seconds() + config.cell_cooldown;

    
    if to_be_rewarded {
        cell.random = block_random[0];
    } else {
        cell.random = std::cmp::min(cell.random + 1, u8::MAX);
    }


    CELLS.insert(deps.storage, &cell_id, &cell)?;
    USER_COOLDOWNS.insert(deps.storage, &sender, &user_cooldown_ends_at)?;
    
    let mut msgs = Vec::<CosmosMsg>::with_capacity(1);

    let mut attributes = vec![
        Attribute {
            key: String::from("cell_id"),
            value: cell_id.to_string(),
            encrypted: false
        },
        Attribute {
            key: String::from("rewarded"),
            value: to_be_rewarded.to_string(),
            encrypted: false
        },
        Attribute {
            key: String::from("can_open_cell_at"),
            value: cell.open_at.to_string(),
            encrypted: false
        },
        Attribute {
            key: String::from("can_open_next_at"),
            value: user_cooldown_ends_at.to_string(),
            encrypted: true
        },

        Attribute {
            key: String::from("cell_score"),
            value: cell_old_random.to_string(),
            encrypted: true
        },

        Attribute {
            key: String::from("user_score"),
            value: user_random.to_string(),
            encrypted: true
        }
    ];


    if to_be_rewarded {
        reward(
            deps, 
            env, 
            sender.clone(), 
            coin.denom,
            &mut msgs, 
            &mut attributes
        )?;
    }


    Ok(
        Response::default()
        .add_event(
            Event::new("cell-opening")
            .add_attributes(attributes)
        )   
        .add_messages(msgs)
    )
}
