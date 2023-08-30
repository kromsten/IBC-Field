use cosmwasm_std::{Deps, Env, StdResult};
use secret_toolkit::permit::Permit;

use crate::state::PERMITS_KEY;

pub fn address_from_permit(deps: Deps, env: &Env, permit: &Permit) -> StdResult<String> {
    secret_toolkit::permit::validate(
        deps,
        PERMITS_KEY,
        &permit,
        env.contract.address.to_string(),
        None,
    )
}
