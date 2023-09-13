use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Cell {0} is not found")]
    NotFound(u8),

    #[error("A dublicate of an powerup was submitted")]
    NotUnique{},

    #[error("Payments info for this token is not configurex")]
    NoAmountInfo{},

    #[error("This IBC channel is not supported")]
    NotSupportedChannel{},

    #[error("Invalid field size. Must be between 3 and 64 and even")]
    InvalidFieldSize{},

    #[error("Only one denom a time is allowed")]
    TooManyDenoms{},

    #[error("Cell is on cooldown. Need to wait for {0} more seconds")]
    CellCooldown(u64),

    #[error("Can't dig anymore for a while. Need to wait for {0} more seconds")]
    UserCooldown(u64),

    #[error("To open the cell with the token you must send {0} units, but the you sent {1} ")]
    NotPaidEnough(u128, u128),

    #[error("Not enough funds to reward the winner. The Reward is {0}, but the contract only has {1} ")]
    NotEnoughFundsNative(u128, u128),

    #[error("Semver parsing error: {0}")]
    SemVer(String),

    #[error("Amount is too big")]
    AmountOverflow {},
}


impl From<semver::Error> for ContractError {
    fn from(err: semver::Error) -> Self {
        Self::SemVer(err.to_string())
    }
}