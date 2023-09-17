#[cfg(test)]
mod tests {
    use std::vec;
    use cosmwasm_std::{Env, DepsMut, MessageInfo, Binary, coins, Coin, Attribute, to_binary, from_binary, Deps};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use secret_toolkit::permit::{Permit, TokenPermissions, PermitParams, PermitSignature, PubKey};
    use crate::{msg::{InstantiateMsg, ExecuteMsg, QueryMsg}, contract::{instantiate, execute, ONE_DAY, query}, state::{NetworkConfig, Powerup}, error::ContractError};
    
    

    pub fn init(deps: DepsMut, env: &Env, info: &MessageInfo) {
        let msg = InstantiateMsg {
            cell_cooldown: None,
            user_cooldown: None,
            win_threshold: None,
            field_size: None,
            network_configs: vec![
                (
                    String::from("uscrt"), 
                    NetworkConfig { 
                        to_win: 1000000u128, 
                        to_open: 50000u128,
                        power_ups: vec![
                            (Powerup::Shovel, 25000u128),
                            (Powerup::Fertilizer, 60000u128),
                            (Powerup::Clover, 100000u128),
                        ],
                        chain_id: "test-1".to_string(),
                        channel_id: None,
                        hrp: None
                    }
                ),
                (
                    String::from("uakt"), 
                    NetworkConfig { 
                        to_win: 2000000u128, 
                        to_open: 20000u128,
                        power_ups: vec![
                            (Powerup::Shovel, 20000u128),
                            (Powerup::Fertilizer, 30000u128),
                            (Powerup::Clover, 50000u128),
                        ],
                        chain_id: "test-2".to_string(),
                        channel_id: None,
                        hrp: None
                    }
                ),
            ]
        };
        //let info = mock_info("creator", &[]);
        let res = instantiate(deps, env.clone(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());
    }
    
    
    pub fn get_permit() -> Permit {
        Permit {
            params: PermitParams {
                allowed_tokens: vec!["cosmos2contract".to_string()],
                permit_name: "my-permit".to_string(),
                chain_id: "test-2".to_string(),
                permissions: vec![TokenPermissions::Owner]
            },
            signature: PermitSignature {
                pub_key: PubKey {
                    r#type: "tendermint/PubKeySecp256k1".to_string(),
                    value: Binary::from_base64("AgGQoJ1UiOfUW1PKCAnoYS+JM9efvuIUTjjmFO7/Y+MZ").unwrap(),
                },
                signature: Binary::from_base64("GKVOykgcmJgmrovzxoCUf7E9dTwemsdTUy2hnQe20phQG88OHtK3Sei4Or8gIe5VRFGh8jwMkqCr9adITJn5Rw==").unwrap()
            }
        }
    }
    
    pub fn get_my_powerups(deps: Deps, env: &Env) -> Vec<(Powerup, u8)> {
        let my = query(deps, env.clone(), QueryMsg::GetMyPowerups { permit: get_permit() });
        let myh: Vec::<(Powerup, u8)> = from_binary(&my.unwrap()).unwrap();
        myh
    }
    
    
    
    #[test]
    fn test_try_opening_cell() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        init(deps.as_mut(), &env, &info);
    
        let msg = ExecuteMsg::OpenCell { 
            cell_id: 1, 
            permit: get_permit(), 
            powerups: vec![],
            power_up_autopay: None, 
        };
    
        let res = execute(deps.as_mut(), env, info, msg);
        assert_eq!(res.unwrap_err(), ContractError::NotPaidAtAll {})
    }
    
    
    #[test]
    fn test_open_error_cases() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        init(deps.as_mut(), &env, &info);
    
    
        let msg_errors : Vec::<(
                ExecuteMsg, 
                Vec<Coin>,
                ContractError
            )> = vec![
    
            (
                ExecuteMsg::OpenCell { 
                    cell_id: 1, 
                    permit: get_permit(), 
                    powerups: vec![],
                    power_up_autopay: None, 
                },
                vec![],
                ContractError::NotPaidAtAll {}
            ),
    
            (
                ExecuteMsg::OpenCell { 
                    cell_id: 1, 
                    permit: get_permit(), 
                    powerups: vec![],
                    power_up_autopay: None, 
                }, 
                coins(10, "uscrt"),
                ContractError::NotPaidEnough(50000, 10)
            ),
    
            (
                ExecuteMsg::OpenCell { 
                    cell_id: 1, 
                    permit: get_permit(), 
                    powerups: vec![],
                    power_up_autopay: None, 
                }, 
                coins(10, "uakt"),
                ContractError::NotPaidEnough(20000, 10)
            ),
    
            (
                ExecuteMsg::OpenCell { 
                    cell_id: 0, 
                    permit: get_permit(), 
                    powerups: vec![],
                    power_up_autopay: None, 
                }, 
                coins(10, "uakt"),
                ContractError::InvalidCellId{}
            ),
    
            (
                ExecuteMsg::OpenCell { 
                    cell_id: 65, 
                    permit: get_permit(), 
                    powerups: vec![],
                    power_up_autopay: None, 
                }, 
                coins(20000, "uakt"),
                ContractError::InvalidCellId {}
            ),
    
            (
                ExecuteMsg::OpenCell { 
                    cell_id: 65, 
                    permit: get_permit(), 
                    powerups: vec![],
                    power_up_autopay: None, 
                }, 
                coins(20000, "uakt"),
                ContractError::InvalidCellId {}
            ),
    
        ];
        
        msg_errors.into_iter().for_each(|(msg, funds, error)| {
            let info = mock_info("alice", &funds);
            let res = execute(deps.as_mut(), env.clone(), info.clone(), msg);
            assert_eq!(res.unwrap_err(), error)
        })
    }
    
    
    
    #[test]
    fn test_opening_work() {
        let mut deps = mock_dependencies();
        let env = mock_env();
    
        let info = mock_info("creator", &coins(20000, "uakt"));
        init(deps.as_mut(), &env, &info);
    
    
        let msg = ExecuteMsg::OpenCell { 
            cell_id: 1, 
            permit: get_permit(), 
            powerups: vec![],
            power_up_autopay: None, 
        };
        
        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        let event = res.events.first().unwrap();
    
        assert_eq!(
            event.attributes.first().unwrap(), 
            Attribute{ key: String::from("cell_id"), value: String::from("1"), encrypted: false}
        );
        assert_eq!(
            event.attributes[1].key, 
            String::from("rewarded")
        );
    
        assert_eq!(
            event.attributes[2],
            Attribute{ key: 
                String::from("can_open_cell_at"), 
                value: (env.block.time.seconds() + 2 * ONE_DAY).to_string(),
                encrypted: false
            }
        );
    
        assert_eq!(
            event.attributes[3],
            Attribute{ key: 
                String::from("can_open_next_at"), 
                value: (env.block.time.seconds() + ONE_DAY).to_string(),
                encrypted: true
            }
        );
    
    }
    
    
    #[test]
    fn test_cell_cooldown() {
        let mut deps = mock_dependencies();
        let env = mock_env();
    
        let info = mock_info("creator", &coins(20000, "uakt"));
        init(deps.as_mut(), &env, &info);
    
    
        let msg = ExecuteMsg::OpenCell { 
            cell_id: 1, 
            permit: get_permit(), 
            powerups: vec![],
            power_up_autopay: None, 
        };
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    
    
        let msg = ExecuteMsg::OpenCell { 
            cell_id: 1, 
            permit: get_permit(), 
            powerups: vec![],
            power_up_autopay: None, 
        };
        let error = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
        assert_eq!(error, ContractError::CellCooldown(2*ONE_DAY))
    }
    
    
    #[test]
    fn test_user_cooldown() {
        let mut deps = mock_dependencies();
        let env = mock_env();
    
        let info = mock_info("creator", &coins(20000, "uakt"));
        init(deps.as_mut(), &env, &info);
    
    
        let msg = ExecuteMsg::OpenCell { 
            cell_id: 1, 
            permit: get_permit(), 
            powerups: vec![],
            power_up_autopay: None, 
        };
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    
    
        let msg = ExecuteMsg::OpenCell { 
            cell_id: 2, 
            permit: get_permit(), 
            powerups: vec![],
            power_up_autopay: None, 
        };
        let error = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
        assert_eq!(error, ContractError::UserCooldown(ONE_DAY))
    }
    
    
    #[test]
    fn test_cooldown_passing() {
        let mut deps = mock_dependencies();
        let mut env = mock_env();
    
        let info = mock_info("creator", &coins(20000, "uakt"));
        init(deps.as_mut(), &env, &info);
    
    
        let first_cell = ExecuteMsg::OpenCell { 
            cell_id: 1, 
            permit: get_permit(), 
            powerups: vec![],
            power_up_autopay: None, 
        };
        
        
        let second_cell = ExecuteMsg::OpenCell { 
            cell_id: 2, 
            permit: get_permit(), 
            powerups: vec![],
            power_up_autopay: None, 
        };
    
        assert!(execute(deps.as_mut(), env.clone(), info.clone(), first_cell.clone()).is_ok());
        // user cooldown
        assert!(execute(deps.as_mut(), env.clone(), info.clone(), second_cell.clone()).is_err());
    
        // user cooldown ends
        env.block.time = env.block.time.plus_seconds(ONE_DAY);
    
        // cell cooldown
        assert!(execute(deps.as_mut(), env.clone(), info.clone(), first_cell.clone()).is_err());
    
        assert!(execute(deps.as_mut(), env.clone(), info.clone(), second_cell.clone()).is_ok());
    
        // first cell cooldown ends
        env.block.time = env.block.time.plus_seconds(ONE_DAY);
    
        // cell cooldown
        assert!(execute(deps.as_mut(), env.clone(), info.clone(), second_cell.clone()).is_err());
    
        assert!(execute(deps.as_mut(), env.clone(), info.clone(), first_cell.clone()).is_ok());
    }
    
    
    #[test]
    fn test_cooldown_powerups() {
        let mut deps = mock_dependencies();
        let env = mock_env();
    
        let info = mock_info("creator", &coins(20000, "uakt"));
        init(deps.as_mut(), &env, &info);
    
        let buy_info = mock_info("creator", &coins(
            20000 + 20000 * 7 + 30000 * 6, "uakt"
        ));
        let buy_powerups = ExecuteMsg::BuyPowerups { 
            powerups: vec![
                Powerup::Shovel,
                Powerup::Shovel,
                Powerup::Shovel,
                Powerup::Shovel,
                Powerup::Shovel,
                Powerup::Shovel,
                Powerup::Shovel,
                Powerup::Fertilizer,
                Powerup::Fertilizer,
                Powerup::Fertilizer,
                Powerup::Fertilizer,
                Powerup::Fertilizer,
                Powerup::Fertilizer
            ],
            permit: get_permit()
        };
    
        let my = query(deps.as_ref(), env.clone(), QueryMsg::GetMyPowerups { permit: get_permit() });
        assert_eq!(my.unwrap(), to_binary(&Vec::<(Powerup, u8)>::new()) .unwrap() );
        assert_eq!(get_my_powerups(deps.as_ref(), &env), vec![]);
        
        assert!(execute(deps.as_mut(), env.clone(), buy_info, buy_powerups.clone()).is_ok());
    
        assert_eq!(get_my_powerups(deps.as_ref(), &env), vec![
            (Powerup::Shovel, 7),
            (Powerup::Fertilizer, 6)
        ]);
    
        let first_normal = ExecuteMsg::OpenCell { 
            cell_id: 1, 
            permit: get_permit(), 
            powerups: vec![],
            power_up_autopay: None, 
        };
        let first_with_shovel = ExecuteMsg::OpenCell { 
            cell_id: 1, 
            permit: get_permit(), 
            powerups: vec![Powerup::Shovel] ,
            power_up_autopay: None,
        };
        let second_with_shovel = ExecuteMsg::OpenCell { 
            cell_id: 2, 
            permit: get_permit(), 
            powerups: vec![Powerup::Shovel],
            power_up_autopay: None,
        };
        let first_with_fert = ExecuteMsg::OpenCell { 
            cell_id: 1, 
            permit: get_permit(), 
            powerups: vec![Powerup::Fertilizer],
            power_up_autopay: None, 
        };
        let second_with_fert = ExecuteMsg::OpenCell { 
            cell_id: 2, 
            permit: get_permit(), 
            powerups: vec![Powerup::Fertilizer],
            power_up_autopay: None, 
        };
        let first_with_both = ExecuteMsg::OpenCell { 
            cell_id: 1, 
            permit: get_permit(), 
            powerups: vec![Powerup::Shovel, Powerup::Fertilizer],
            power_up_autopay: None, 
        };
        let second_with_both = ExecuteMsg::OpenCell { 
            cell_id: 2, 
            permit: get_permit(), 
            powerups: vec![Powerup::Shovel,Powerup::Fertilizer],
            power_up_autopay: None, 
        };
        
    
        assert!(execute(deps.as_mut(), env.clone(), info.clone(), first_normal.clone()).is_ok());
    
        assert_eq!(get_my_powerups(deps.as_ref(), &env), vec![
            (Powerup::Shovel, 7),
            (Powerup::Fertilizer, 6)
        ]);    
    
        // user cooldown
        assert!(execute(deps.as_mut(), env.clone(), info.clone(), first_with_fert.clone()).is_err());
    
        // Mock storage doesn't revert on error
        // Poweups are being consumed @_@
        assert_eq!(get_my_powerups(deps.as_ref(), &env), vec![
            (Powerup::Shovel, 7),
            (Powerup::Fertilizer, 5)
        ]);
    
        assert!(execute(deps.as_mut(), env.clone(), info.clone(), second_with_fert.clone()).is_err());
        
        // cell cooldown
        assert!(execute(deps.as_mut(), env.clone(), info.clone(), first_with_shovel.clone()).is_err());
    
        assert_eq!(get_my_powerups(deps.as_ref(), &env), vec![
            (Powerup::Shovel, 6),
            (Powerup::Fertilizer, 4)
        ]);
    
        assert!(execute(deps.as_mut(), env.clone(), info.clone(), second_with_shovel.clone()).is_ok());
        
        assert_eq!(get_my_powerups(deps.as_ref(), &env), vec![
            (Powerup::Shovel, 5),
            (Powerup::Fertilizer, 4)
        ]);
    
        assert!(execute(deps.as_mut(), env.clone(), info.clone(), first_with_both.clone()).is_ok());
        
        assert_eq!(get_my_powerups(deps.as_ref(), &env), vec![
            (Powerup::Shovel, 4),
            (Powerup::Fertilizer, 3)
        ]);
    
        assert!(execute(deps.as_mut(), env.clone(), info.clone(), second_with_both.clone()).is_ok());
        assert_eq!(get_my_powerups(deps.as_ref(), &env), vec![
            (Powerup::Shovel, 3),
            (Powerup::Fertilizer, 2)
        ]);
    
        assert!(execute(deps.as_mut(), env.clone(), info.clone(), second_with_both.clone()).is_ok());
        assert!(execute(deps.as_mut(), env.clone(), info.clone(), second_with_both.clone()).is_ok());
    
        assert_eq!(get_my_powerups(deps.as_ref(), &env), vec![
            (Powerup::Shovel, 1),
            (Powerup::Fertilizer, 0)
        ]);
    
        let res =  execute(deps.as_mut(), env.clone(), info.clone(), second_with_both.clone());
        assert_eq!(res.unwrap_err(), ContractError::NoPowerups{});
    
    
        let auto_pay_second_with_both = ExecuteMsg::OpenCell { 
            cell_id: 2, 
            permit: get_permit(), 
            powerups: vec![Powerup::Shovel,Powerup::Fertilizer],
            power_up_autopay: Some(true), 
        };
        let auto_pay_info = mock_info("creator", &coins(
            20000 * 2 + 30000, "uakt"
        ));
    
        assert!(execute(deps.as_mut(), env.clone(), auto_pay_info.clone(), auto_pay_second_with_both.clone()).is_ok());
    
    
        let buy_info = mock_info("creator", &coins(
            20000 + 20000, "uakt"
        ));
        let buy_powerups = ExecuteMsg::BuyPowerups { 
            powerups: vec![
                Powerup::Shovel,
            ],
            permit: get_permit()
        };
        assert!(execute(deps.as_mut(), env.clone(), buy_info, buy_powerups.clone()).is_ok());
    
    
    
        let auto_pay_second_with_both = ExecuteMsg::OpenCell { 
            cell_id: 2, 
            permit: get_permit(), 
            powerups: vec![Powerup::Shovel,Powerup::Fertilizer],
            power_up_autopay: Some(true), 
        };
        let auto_pay_info = mock_info("creator", &coins(
            20000 + 30000, "uakt"
        ));
    
        assert!(execute(deps.as_mut(), env.clone(), auto_pay_info.clone(), auto_pay_second_with_both.clone()).is_ok());
    
    
    }
}


