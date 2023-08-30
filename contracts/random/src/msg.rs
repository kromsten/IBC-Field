use cosmwasm_schema::cw_serde;
use secret_toolkit::permit::Permit;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateMyRandomNumber {
        permit: Permit
    },
}

#[cw_serde]
pub enum QueryMsg {
    GetMyRandomNumber {
        permit: Permit
    }
}
