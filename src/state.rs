use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{ Item, Map };

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct PartnerInfo {
    pub owner : Addr,
    pub protocol_fee : u128,
    pub fee_receiver : Option<Addr>,
}


pub const FEE_RECEIVER: Item<Option<Addr>> = Item::new("feeReceiver");
pub const PARTNER_INFOS:  Map<String, PartnerInfo> =  Map::new("partner_infos");
pub const WHITE_LIST_TOKENS: Map<String, bool> =  Map::new("white list tokens");
pub const IS_ACTIVED: Map<Addr, bool> = Map::new("is actived");