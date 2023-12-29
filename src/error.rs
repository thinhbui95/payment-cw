
use thiserror::Error;
use cosmwasm_std::StdError;
use cw_ownable::OwnershipError;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error(transparent)]
    StdError(#[from] StdError),
  
    #[error(transparent)]
    Ownership(#[from] OwnershipError),
    
    #[error("AdminRole: Not an admin")]
    NotAdmins{},

    #[error("Partner does not exit")]
    PartnerNotExit{},

    #[error("You are not owner")]
    Unauthorized{},

    #[error("Len doesn't match")]
    NotMatch{},

    #[error("Invalid amount")]
    InvalidAmount{},

    #[error("Invalid token")]
    InvalidToken,

    #[error("Partner exist")]
    PartnerExist,
}