use crate::{state::*, error::*, msg::*};
use cosmwasm_std::{Deps, Addr};
use cosmwasm_std::StdError;

pub fn get_partner_info(
    deps: Deps, 
    msg: GetPartnerInfoMsg
) -> Result<PartnerInfo, ContractError> {
    Ok(PARTNER_INFOS.load(deps.storage, msg.partner_code).unwrap())
}


pub fn get_fee_receiver(
    deps: Deps, 
)-> Result<Option<Addr>, ContractError> {
    Ok(FEE_RECEIVER.load(deps.storage).unwrap())
}

pub fn is_white_list_token(
    deps: Deps, 
    msg: IsWhiteListTokenMsg
)-> Result<bool, ContractError>{
    Ok(WHITE_LIST_TOKENS.load(deps.storage,msg.token_address).unwrap())
}

pub fn query_ownership(
    deps: Deps
  ) -> Result<cw_ownable::Ownership<Addr>, StdError> {
    cw_ownable::get_ownership(deps.storage)
  }