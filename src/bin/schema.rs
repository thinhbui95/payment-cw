use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_std::Addr;
use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use cw_ownable::Ownership;

use payment::{
  msg::{
    InstantiateMsg,
    ExecuteMsg,
    PaymentQueryMsg,
    PartnerInfoRespone


  },
  state::{
    MigrateMsg,
    PartnerInfo,
  },
};


fn main() {
  let mut out_dir = current_dir().unwrap();
  out_dir.push("schema");
  create_dir_all(&out_dir).unwrap();
  remove_schemas(&out_dir).unwrap();

  export_schema(&schema_for!(InstantiateMsg), &out_dir);
  export_schema(&schema_for!(ExecuteMsg), &out_dir);
  export_schema(&schema_for!(PaymentQueryMsg), &out_dir);
  export_schema(&schema_for!(Ownership<Addr>), &out_dir);
  export_schema(&schema_for!(PartnerInfoRespone), &out_dir);
  export_schema(&schema_for!(MigrateMsg), &out_dir);
  export_schema(&schema_for!(PartnerInfo), &out_dir);

}
