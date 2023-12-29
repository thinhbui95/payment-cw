#[cfg(test)]
pub mod tests {
  use cw_multi_test::{ App, Contract, ContractWrapper, Executor };
  use cosmwasm_std::{Empty,Uint128,Coin};
  use crate::{utils_test::*, msg::*};

  pub fn payment_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
      crate::contract::execute,
      crate::contract::instantiate,
      crate::contract::query
    );
    Box::new(contract)
  }

#[test]
pub fn test() {
  const OWNER: &str = "owner";
  const FEE_RECEIVER: &str = "fee_receiver";
  const NEW_FEE_RECEIVER: &str = "new_fee_receiver";
  const ADMIN1: &str = "adimin1";
  const ADMIN2: &str = "adimin2";
  const PARTNER1: &str = "partner1";
  const NEW_PARTNER1: &str = "new_partner1";
  const FEE_PARTNER_RECEIVER1: &str = "fee_partner_receiver1";
  const NEW_FEE_PARTNER_RECEIVER1: &str = "new_fee_partner_receiver1";
  // const PARTNER2: &str = "partner2";
  // const FEE_PARTNER_RECEIVER2: &str = "fee_partner_receiver2";

  let mut app = App::default();
  let payment_code_id = app.store_code(payment_contract());


  //Init contract 
  let payment = app.instantiate_contract(
    payment_code_id,
    get_uaddr(OWNER),
    &(InstantiateMsg {
      fee_receiver:Some(get_uaddr(FEE_RECEIVER)),
    }),
    &[],
    "Payment",
    None
  ).unwrap();

  
  let res = mint_native_token(&mut app,&payment.clone(), 10000000, "btc").expect("msg");
  mint_native_token(&mut app,&get_uaddr(PARTNER1), 10000000, "btc").expect("msg");
  mint_native_token(&mut app, &get_uaddr(NEW_PARTNER1), 10000000, "btc").expect("msg");
  mint_native_token(&mut app, &get_uaddr(ADMIN1), 10000000, "btc").expect("msg");

  let denom  = &res[0].denom;
  println!("denom token {:?}", denom );


  let balance_new_partner1_before = app.wrap().query_balance(&get_uaddr(NEW_PARTNER1), "btc").unwrap().amount;
  let balance_new_fee_partner_receiver1_before = app.wrap().query_balance(&get_uaddr(NEW_FEE_PARTNER_RECEIVER1), "btc").unwrap().amount;
  let balance_fee_receiver_before = app.wrap().query_balance(&get_uaddr(NEW_FEE_RECEIVER), "btc").unwrap().amount;

  println!("balance of partner before {:?}", balance_new_partner1_before);
  println!("balance of fee partner receiver before {:?}", balance_new_fee_partner_receiver1_before);
  println!("balance of fee receiver before {:?}", balance_fee_receiver_before);
  
  //Set admins with owner
  app.execute_contract(get_uaddr(OWNER), payment.clone(),&(ExecuteMsg::SetAdmins {msg: SetAdminsMsg{
    admins: vec![get_uaddr(ADMIN1),get_uaddr(ADMIN2) ],
    is_actives: vec![true, false]
  }}),
  &[]
  ).unwrap();


  //Update fee receiver
    app.execute_contract(get_uaddr(OWNER),payment.clone(),&(ExecuteMsg::UpdateFeeReceiver {msg: UpdateFeeReceiverMsg{
    new_fee_receiver:Some(get_uaddr(NEW_FEE_RECEIVER))
  }}),
  &[]
  ).unwrap();

 
  //Create a partner
  app.execute_contract(get_uaddr(ADMIN1),payment.clone(),&(ExecuteMsg::CreatePartner {msg: CreatePartnerMsg{
    partner_code: "1234".to_string(),
    owner: get_uaddr(PARTNER1),
    protocol_fee: 1000,
    fee_receiver : Some(get_uaddr(FEE_PARTNER_RECEIVER1))
  }}),
  &[]
  ).unwrap();

  //Update partner protocol fee 
  app.execute_contract(get_uaddr(ADMIN1), payment.clone(), &(ExecuteMsg::UpdatePartnerProtocolFee {msg: UpdatePartnerProtocolFeeMsg{
    partner_code: "1234".to_string(),
    new_protocol_fee : 2000
  }}),
  &[]
  ).unwrap();

  //Update partner owner
  app.execute_contract(get_uaddr(PARTNER1), payment.clone(), &(ExecuteMsg::UpdatePartnerOwner {msg: UpdatePartnerOwnerMsg{
    partner_code: "1234".to_string(),
    new_owner : get_uaddr(NEW_PARTNER1)
  }}),
  &[]
  ).unwrap();

  //Update Partner Fee Receiver
   app.execute_contract(get_uaddr(NEW_PARTNER1), payment.clone(), &(ExecuteMsg::UpdatePartnerFeeReceiver {msg: UpdatePartnerFeeReceiverMsg{
    partner_code: "1234".to_string(),
    new_fee_receiver : Some(get_uaddr(NEW_FEE_PARTNER_RECEIVER1))
  }}),
  &[]
  ).unwrap();

  //set whitelist token
  app.execute_contract(get_uaddr(ADMIN1), payment.clone(), &(ExecuteMsg::SetWhitelistTokens {msg: SetWhitelistTokensMsg {
    tokens : vec![denom.to_string()],
    is_actives: vec![true]
  }}),
  &[]
  ).unwrap();


  //Paymaster 
  app.execute_contract(get_uaddr(NEW_PARTNER1), payment.clone(), &(ExecuteMsg::Pay {msg: PayMsg {
    partner_code: "1234".to_string(),
    is_native_token: true,
    token_addr: denom.to_string(),
    amount: Uint128::from(100u128),
    pay_for: None,
    data: None
  }}),
  &[Coin {
    amount: Uint128::from(100u128),
    denom: "btc".to_string(),
  }]
  ).unwrap();

  
  let balance_new_partner1_after = app.wrap().query_balance(&get_uaddr(NEW_PARTNER1), "btc").unwrap().amount;
  let balance_new_fee_partner_receiver1_after = app.wrap().query_balance(&get_uaddr(NEW_FEE_PARTNER_RECEIVER1), "btc").unwrap().amount;
  let balance_fee_receiver_after = app.wrap().query_balance(&get_uaddr(NEW_FEE_RECEIVER), "btc").unwrap().amount;
  
  println!("balance of partner after {:?}", balance_new_partner1_after);
  println!("balance of fee partner receiver after {:?}", balance_new_fee_partner_receiver1_after);
  println!("balance of fee receiver after {:?}", balance_fee_receiver_after);



  //withdraw fungible token
  let balance_owner_before = app.wrap().query_balance(&get_uaddr(OWNER), "btc").unwrap().amount;
  println!("balance of owner before {:?}", balance_owner_before);
  app.execute_contract(get_uaddr(OWNER), payment.clone(), &(ExecuteMsg::WithdrawFungibleToken {msg: WithdrawFungibleTokenMsg {
    token_addr: denom.to_string(),
    amount: Uint128::from(10u128),
    is_native_token: true
  }}),
  &[]
  ).unwrap();

  let balance_owner_after = app.wrap().query_balance(&get_uaddr(OWNER), "btc").unwrap().amount;
  println!("balance of owner after {:?}", balance_owner_after);


}
}