
use cw_ownable::{cw_ownable_execute, cw_ownable_query};
use cosmwasm_std::{Addr, Uint128};
use cosmwasm_schema::{ cw_serde, QueryResponses };


#[cw_serde]
pub struct InstantiateMsg {
    pub fee_receiver : Option<Addr>
}

#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    UpdateFeeReceiver {
        msg:  UpdateFeeReceiverMsg
    },
    SetAdmins {
        msg: SetAdminsMsg
    },

    CreatePartner {
        msg: CreatePartnerMsg
    },

    DeletePartner {
        msg: DeletePartnerMsg
    },

    UpdatePartnerProtocolFee {
        msg: UpdatePartnerProtocolFeeMsg
    },

    SetWhitelistTokens {
        msg: SetWhitelistTokensMsg
    },

    UpdatePartnerOwner {
        msg: UpdatePartnerOwnerMsg
    },
 
    UpdatePartnerFeeReceiver {
        msg: UpdatePartnerFeeReceiverMsg
    },

    Pay {
        msg: PayMsg
    },

    WithdrawFungibleToken {
        msg: WithdrawFungibleTokenMsg
    },

    WithdrawNft {
        msg: WithdrawNftMsg
    },
}

#[cw_serde]
pub struct CreatePartnerMsg {
    pub partner_code: String,
    pub owner: Addr,
    pub protocol_fee: u128,
    pub fee_receiver : Option<Addr>
}

#[cw_serde]
pub struct UpdateFeeReceiverMsg {
    pub new_fee_receiver: Option<Addr>
}

#[cw_serde]
pub struct SetAdminsMsg {
    pub admins : Vec<Addr>,
    pub is_actives: Vec<bool>
}

#[cw_serde]
pub struct DeletePartnerMsg {
    pub partner_code: String,
}

#[cw_serde]
pub struct UpdatePartnerProtocolFeeMsg {
    pub partner_code: String,
    pub new_protocol_fee : u128
}

#[cw_serde]
pub struct SetWhitelistTokensMsg {
    pub tokens : Vec<String>,
    pub is_actives: Vec<bool>
}

#[cw_serde]
pub struct UpdatePartnerOwnerMsg {
    pub partner_code: String,
    pub new_owner : Addr
}

#[cw_serde]
pub struct UpdatePartnerFeeReceiverMsg {
    pub partner_code: String,
    pub new_fee_receiver : Option<Addr>
}

#[cw_serde]
pub struct PayMsg {
    pub partner_code: String,
    pub is_native_token: bool,
    pub token_addr: String,
    pub amount: Uint128,
    pub pay_for: Option<Addr>,
    pub data: Option<[u8;32]>
}

#[cw_serde]
pub struct WithdrawFungibleTokenMsg {
    pub token_addr: String,
    pub amount: Uint128,
    pub is_native_token: bool
}

#[cw_serde]
pub struct WithdrawNftMsg {
    pub token_address: Addr,
    pub token_ids: Vec<String>
}

#[cw_serde]
pub struct GetPartnerInfoMsg {
    pub partner_code: String
}

#[cw_serde]
pub struct IsWhiteListTokenMsg {
    pub token_address: String
}

#[cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum PaymentQueryMsg  {
    #[returns(PartnerInfoRespone)] GetPartnerInfo {
        msg: GetPartnerInfoMsg
    },
    #[returns(Addr)] GetFeeReceiver {},
    #[returns(bool)] IsWhiteListToken {
        msg: IsWhiteListTokenMsg,
    },
}

#[cw_serde]
pub struct PartnerInfoRespone { 
    pub owner: Addr,
    pub protocol_fee: u128,
    pub fee_receiver: Option<Addr>
}







