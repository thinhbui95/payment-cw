use cw20::Cw20Coin;
use cosmwasm_std::{ DepsMut, Env, Event,ensure,MessageInfo,Response,Attribute,Uint128};
use crate::{ error::*, state::* ,utils::*, msg::*};

/**
* @dev Update fee receiver
*/
pub fn update_fee_receiver(
    deps: DepsMut,
    info: MessageInfo,
    msg:  UpdateFeeReceiverMsg
) -> Result<Response, ContractError> {
    cw_ownable::assert_owner(deps.storage, &info.sender)?;
    FEE_RECEIVER.save(deps.storage, &msg.new_fee_receiver)?;
    let event = Event::new("update_receiver_addr").add_attribute("receiver_addr", msg.new_fee_receiver.clone().map(|addr| addr.to_string()).unwrap_or_else(|| "Empty address".to_string()));
    
    Ok(Response::new().add_event(event))
}

/**
* @dev Set admins list
*/
pub fn set_admins(
    deps: DepsMut,
    info: MessageInfo,
    msg: SetAdminsMsg,
) -> Result<Response, ContractError> {
    cw_ownable::assert_owner(deps.storage, &info.sender)?;
    let mut res = Response::new();
    ensure!(msg.admins.len() == msg.is_actives.len(), ContractError::NotMatch{});
    _set_admins(msg.admins, msg.is_actives,deps, &mut res)?;
    Ok(res)
}

/**
* @dev Create new partner
*/
pub fn create_partner(
    deps: DepsMut,
    info: MessageInfo,
    msg: CreatePartnerMsg,
) -> Result<Response, ContractError> {
    let is_actived = IS_ACTIVED.may_load(deps.storage, info.sender)?;
    ensure!(is_actived.is_some() && is_actived.unwrap()  , ContractError::NotAdmins{});
    
    let parner_info = PARTNER_INFOS.may_load(deps.storage,msg.partner_code.clone())?;
    ensure!(!parner_info.is_some(),ContractError::PartnerExist{});

    let event = Event::new("partner_created").add_attributes(
        vec![
            ("partnerCode", msg.partner_code.clone()),
            ("owner", msg.owner.to_string()),
            ("protocolFee", msg.protocol_fee.to_string()),
            ("feeReceiver",msg.fee_receiver.clone().map(|addr| addr.to_string()).unwrap_or_else(|| "Empty address".to_string()))
        ]
    );
    PARTNER_INFOS.save(deps.storage, msg.partner_code,&PartnerInfo{
        owner: msg.owner,
        protocol_fee : msg.protocol_fee,
        fee_receiver : msg.fee_receiver})?;
    
    Ok(Response::new().add_event(event))
}

/**
* @dev Delete a partner
*/
pub fn delete_partner(
    deps: DepsMut,
    info: MessageInfo,
    msg: DeletePartnerMsg,
) -> Result<Response, ContractError> {
    let is_actived = IS_ACTIVED.may_load(deps.storage, info.sender)?;
    ensure!(is_actived.is_some() && is_actived.unwrap(), ContractError::NotAdmins{});
    
    let  partner_info = PARTNER_INFOS.may_load(deps.storage,msg.partner_code.clone())?;
    ensure!(partner_info.is_some(), ContractError::PartnerNotExit{});
    PARTNER_INFOS.remove(deps.storage, msg.partner_code.clone());
    
    let event = Event::new("delete_partner").add_attribute("delete_partner", msg.partner_code.clone());
    Ok(Response::new().add_event(event))
}


/**
* @dev Update protocol fee (Only Admins)
*/
pub fn update_protocol_fee(
    deps: DepsMut,
    info: MessageInfo,
    msg: UpdatePartnerProtocolFeeMsg
) -> Result<Response, ContractError> {
    let is_actived = IS_ACTIVED.may_load(deps.storage, info.sender)?;
    ensure!(is_actived.is_some() && is_actived.unwrap(), ContractError::NotAdmins{});
    
    let  partner_info = PARTNER_INFOS.may_load(deps.storage,msg.partner_code.clone())?;
    ensure!(partner_info.is_some(), ContractError::PartnerNotExit{});

    let event = Event::new("update_protocol_fee").add_attributes(
        vec![
            ("partnerCode", msg.partner_code.clone()),
            ("newProtocolFee", msg.new_protocol_fee.to_string()),
            ]
        );
    let new_updated = PartnerInfo{
        owner: partner_info.clone().unwrap().owner,
        protocol_fee: msg.new_protocol_fee,
        fee_receiver: partner_info.unwrap().fee_receiver
    };

    PARTNER_INFOS.save(deps.storage,msg.partner_code ,&new_updated)?;
    Ok(Response::new().add_event(event))
}

/**
* @dev Update partner owner (Only partner owner)
*/
pub fn update_partner_owner(
    deps: DepsMut,
    info: MessageInfo,
    msg: UpdatePartnerOwnerMsg
) -> Result<Response, ContractError> {
    let partner_info = PARTNER_INFOS.may_load(deps.storage,msg.partner_code.clone())?;
    ensure!(partner_info.is_some(), ContractError::PartnerNotExit{});
    ensure!(partner_info.clone().unwrap().owner == info.sender, ContractError::Unauthorized{});

    let event = Event::new("update_owner_partner").add_attributes(
        vec![
            ("partnerCode", msg.partner_code.clone()),
            ("newOwner", msg.new_owner.to_string()),
            ]
        );

        let new_updated = PartnerInfo{
            owner: msg.new_owner,
            protocol_fee: partner_info.clone().unwrap().protocol_fee,
            fee_receiver: partner_info.unwrap().fee_receiver
        };
    
    PARTNER_INFOS.save(deps.storage,msg.partner_code ,&new_updated)?;
    Ok(Response::new().add_event(event))
}

/**
* @dev Update partner fee receiver (Only partner owner)
*/
pub fn update_partner_fee_receiver(
    deps: DepsMut,
    info: MessageInfo,
    msg: UpdatePartnerFeeReceiverMsg
) -> Result<Response, ContractError> {
    let partner_info = PARTNER_INFOS.may_load(deps.storage,msg.partner_code.clone())?;
    ensure!(partner_info.is_some(), ContractError::PartnerNotExit{});
    ensure!(partner_info.clone().unwrap().owner == info.sender, ContractError::Unauthorized{});

    let event = Event::new("update_fee_receiver").add_attributes(
            vec![
                ("partnerCode", msg.partner_code.clone()),
                ("newFeeReceiver", msg.new_fee_receiver.clone().map(|addr| addr.to_string()).unwrap_or_else(|| "Empty address".to_string()))
            ]
        );
    let new_updated = PartnerInfo{
            owner: partner_info.clone().unwrap().owner,
            protocol_fee: partner_info.clone().unwrap().protocol_fee,
            fee_receiver: msg.new_fee_receiver
        };
    
    PARTNER_INFOS.save(deps.storage,msg.partner_code ,&new_updated)?;
    Ok(Response::new().add_event(event))
}

/**
* @dev Set white list token (Only Admnins)
*/
pub fn set_white_list_token(
    deps: DepsMut,
    info: MessageInfo,
    msg: SetWhitelistTokensMsg
)-> Result<Response, ContractError> {
    let is_actived = IS_ACTIVED.may_load(deps.storage, info.sender)?;
    ensure!(is_actived.is_some() && is_actived.unwrap(), ContractError::NotAdmins{});
    ensure!(msg.tokens.len() == msg.is_actives.len(), ContractError::NotMatch{});
    
    let mut attrs: Vec<Attribute> = vec![];
    for i in 0..msg.tokens.len() {
        WHITE_LIST_TOKENS.save(deps.storage, msg.tokens[i].clone(), &msg.is_actives[i])?;
        attrs.push(Attribute { key : msg.tokens[i].to_string(), value: msg.is_actives[i].to_string()});
    }
    let event = Event::new("admins_updated").add_attributes(attrs);
    return Ok(Response::new().add_event(event))
}


/**
* @dev Implement payment (Only partner owner)
*/
pub fn pay(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: PayMsg
)-> Result<Response, ContractError> {
    
    let is_whitelist = WHITE_LIST_TOKENS.may_load(deps.storage, msg.token_addr.clone())?;
    ensure!(is_whitelist.is_some() && is_whitelist.unwrap() , ContractError::InvalidToken{});
    let mut res = Response::new();
    
    let partner_info = PARTNER_INFOS.may_load(deps.storage,msg.partner_code.clone())?;
    ensure!(partner_info.is_some(), ContractError::PartnerNotExit{});
    ensure!(partner_info.clone().unwrap().owner == info.sender, ContractError::Unauthorized{});
    
    let token = Cw20Coin{address: msg.token_addr.clone(), amount: msg.amount};


    //Transfer token from sender to contract
    transfer_from_coin(info.clone(),token.clone(),env.contract.address,msg.is_native_token,&mut res)?;
    let protocol_fee = token.amount.checked_mul(Uint128::from(partner_info.clone().unwrap().protocol_fee)).unwrap().checked_div(Uint128::from(10000u128)).unwrap();
   
    let fee_receiver = FEE_RECEIVER.load(deps.storage)?;
    //transfer fee from contract to fee receiver
    if fee_receiver.is_some() {
        let coin_protocol_fee = Cw20Coin{address: token.address.clone(), amount: protocol_fee};
        transfer_coin(info.clone(),coin_protocol_fee,fee_receiver.unwrap(),msg.is_native_token,&mut res)?;
    }
    //transfer fee from contract to fee partner receiver
    if partner_info.clone().unwrap().fee_receiver.is_some() {
        let coin_fee_receiver = Cw20Coin{address: token.address.clone(), amount: msg.amount.clone().checked_sub(protocol_fee).unwrap()};
        transfer_coin(info.clone(),coin_fee_receiver,partner_info.unwrap().fee_receiver.unwrap(), msg.is_native_token,&mut res)?;
    }

    let event = Event::new("Payment").add_attributes(
        vec![
            ("partnerCode", msg.partner_code),
            ("token", msg.token_addr.clone().to_string()),
            ("amount", msg.amount.clone().to_string()),
            ("sender", info.sender.to_string()),
            ("payFor",msg.pay_for.clone().map(|addr| addr.to_string()).unwrap_or_else(|| "Empty address".to_string())),
            ("data", option_bytes_to_string(msg.data))
        ]
    );
    return Ok(res.add_event(event))
}

/**
 * @dev Withdraw fee token for owner
*/
pub fn withdraw_fungible_token(
    deps: DepsMut,
    info: MessageInfo,
    msg: WithdrawFungibleTokenMsg
)-> Result<Response, ContractError>{
    let mut res = Response::new();
    cw_ownable::assert_owner(deps.storage, &info.sender)?;
    let owner = cw_ownable::get_ownership(deps.storage)?;
    let token = Cw20Coin{address: msg.token_addr, amount: msg.amount};
    transfer_coin(info,token,owner.owner.unwrap(),msg.is_native_token,&mut res)?;
    Ok(res)
}


/**
 * @dev Withdraw nft token for owner
*/
pub fn withdraw_non_fungible_token(
    deps: DepsMut,
    info: MessageInfo,
    msg: WithdrawNftMsg
)-> Result<Response, ContractError>{
    cw_ownable::assert_owner(deps.storage, &info.sender)?;
    let mut res = Response::new();
    for i in 0..msg.token_ids.len() {
        transfer_nft(msg.token_address.clone(),msg.token_ids[i].clone(),info.sender.clone(),&mut res)?;
    }
    Ok(res)
}

pub fn execute_update_ownership(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    action: cw_ownable::Action
  ) -> Result<Response, ContractError> {
    let ownership = cw_ownable::update_ownership(deps, &env.block, &info.sender, action)?;

    Ok(Response::new().add_attributes(ownership.into_attributes()))
  }



















