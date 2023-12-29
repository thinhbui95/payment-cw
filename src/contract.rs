use cosmwasm_std::{ DepsMut, Env, MessageInfo, Response, entry_point, Deps, StdResult, Binary, to_json_binary };
use crate::{msg::*, error::*, state::*, execute::*, utils::*,query::*};


// version info for migration info
const CONTRACT_NAME: &str = "Payment";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env, 
    info: MessageInfo,
    msg:InstantiateMsg
)-> Result<Response, ContractError> {
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(&info.sender.to_string()))?;
    FEE_RECEIVER.save(deps.storage,&msg.fee_receiver)?;
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let mut res: Response = Response::new();
    let _ =_set_admin(info.sender,&true,deps, &mut res);
    Ok(res.add_attribute("contract_name", CONTRACT_NAME).add_attribute("contract_version", CONTRACT_VERSION))

}

#[entry_point]
pub fn execute(
    deps: DepsMut, 
    env: Env, 
    info: MessageInfo,
    msg: ExecuteMsg
)-> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePartner { msg:create_partner_msg} => {
            create_partner(deps,info,create_partner_msg)
        }
        ExecuteMsg::DeletePartner { msg: delete_partner_msg } =>{
            delete_partner(deps, info, delete_partner_msg)
        }
        ExecuteMsg::UpdateFeeReceiver{msg:update_fee_receiver_msg} => {
            update_fee_receiver(deps,info, update_fee_receiver_msg)
        }
        ExecuteMsg::SetAdmins {msg: set_admins_msg} => {
            set_admins(deps,info,set_admins_msg)
        }
        ExecuteMsg::UpdatePartnerProtocolFee {msg: update_partner_protocol_fee_msg} => {
            update_protocol_fee(deps,info,update_partner_protocol_fee_msg)
        }
        ExecuteMsg::SetWhitelistTokens { msg: set_white_list_tokens_msg } => {
            set_white_list_token(deps,info,set_white_list_tokens_msg)
        }
        ExecuteMsg::UpdatePartnerOwner { msg: update_partner_owner_msg } => {
            update_partner_owner(deps, info,update_partner_owner_msg)
        }
        ExecuteMsg::UpdatePartnerFeeReceiver { msg: update_partner_fee_receiver_msg} =>{
            update_partner_fee_receiver(deps, info,update_partner_fee_receiver_msg)
        }
        ExecuteMsg::Pay { msg: pay_msg} => {
            pay(deps,env,info,pay_msg)
        }
        ExecuteMsg::WithdrawNft { msg: withdraw_nft_msg} => {
            withdraw_non_fungible_token(deps,info,withdraw_nft_msg)
        }
        ExecuteMsg::WithdrawFungibleToken { msg: withdraw_fungible_token_msg} => {
            withdraw_fungible_token(deps,info,withdraw_fungible_token_msg)
        }
        ExecuteMsg::UpdateOwnership(action) => {
            execute_update_ownership(deps,env,info,action)
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: PaymentQueryMsg) -> StdResult<Binary> {
    match msg {
        PaymentQueryMsg::GetPartnerInfo { msg:  get_partner_info_msg} => {
            to_json_binary(&get_partner_info(deps,get_partner_info_msg).unwrap())
        }

        PaymentQueryMsg::IsWhiteListToken { msg:  is_whitelist_token} => {
            to_json_binary(&is_white_list_token(deps,is_whitelist_token).unwrap())
        }

        PaymentQueryMsg::GetFeeReceiver {} => {
            to_json_binary(&get_fee_receiver(deps).unwrap())
        }

        PaymentQueryMsg::Ownership {  } => {
            to_json_binary(&query_ownership(deps).unwrap())
        }

    }
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
