#[cfg(test)]
pub mod tests {
  use cw_multi_test::{ App, Contract, ContractWrapper, Executor };
  use cw721_base::InstantiateMsg as Cw721ExecuteMsg;

  use cosmwasm_std::{Empty,Uint128};
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

  let users = vec![payment.clone(),get_uaddr(PARTNER1),get_uaddr(NEW_PARTNER1)];
  let token = cw20::Cw20Contract(create_cw20_token(&mut app, &get_uaddr(OWNER),users));


  approve_cw20_token(&mut app,&get_uaddr(PARTNER1),payment.clone(),&token.addr(),Uint128::from(10000u128)).expect("error");
  approve_cw20_token(&mut app,&get_uaddr(NEW_PARTNER1),payment.clone(),&token.addr(),Uint128::from(10000u128)).expect("error");

  let balance_new_partner1_before = token.balance(&mut app.wrap(), get_uaddr(NEW_PARTNER1)).unwrap();
  let balance_new_fee_partner_receiver1_before = token.balance(&mut app.wrap(), get_uaddr(NEW_FEE_PARTNER_RECEIVER1)).unwrap();
  let balance_fee_receiver_before = token.balance(&mut app.wrap(), get_uaddr(NEW_FEE_RECEIVER)).unwrap();

  println!("balance of partner before {:?}", balance_new_partner1_before);
  println!("balance of fee partner receiver before {:?}", balance_new_fee_partner_receiver1_before);
  println!("balance of fee receiver before {:?}", balance_fee_receiver_before);
  
  //Set admins
  app.execute_contract(get_uaddr(OWNER), payment.clone(),&(ExecuteMsg::SetAdmins {msg: SetAdminsMsg{
    admins: vec![get_uaddr(ADMIN1),get_uaddr(ADMIN2) ],
    is_actives: vec![true, false]
  }}),
  &[]
  ).unwrap();

  //Update fee receiver
  app.execute_contract(get_uaddr(OWNER),payment.clone(),&(ExecuteMsg::UpdateFeeReceiver {msg: UpdateFeeReceiverMsg{
    new_fee_receiver:None
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
    new_fee_receiver : None
  }}),
  &[]
  ).unwrap();



  //set whitelist token
  app.execute_contract(get_uaddr(OWNER), payment.clone(), &(ExecuteMsg::SetWhitelistTokens {msg: SetWhitelistTokensMsg {
    tokens : vec![token.addr().to_string()],
    is_actives: vec![true]
  }}),
  &[]
  ).unwrap();

  //Paymaster 
  app.execute_contract(get_uaddr(NEW_PARTNER1), payment.clone(), &(ExecuteMsg::Pay {msg: PayMsg {
    partner_code: "1234".to_string(),
    is_native_token: false,
    token_addr: token.addr().into_string(),
    amount: Uint128::from(100u128),
    pay_for: Some(get_uaddr(NEW_PARTNER1)),
    data: None
  }}),
  &[]
  ).unwrap();




  let balance_new_partner1_after = token.balance(&mut app.wrap(), get_uaddr(NEW_PARTNER1)).unwrap();
  let balance_new_fee_partner_receiver1_after = token.balance(&mut app.wrap(), get_uaddr(NEW_FEE_PARTNER_RECEIVER1)).unwrap();
  let balance_fee_receiver_after = token.balance(&mut app.wrap(), get_uaddr(NEW_FEE_RECEIVER)).unwrap();
  
  println!("balance of partner after {:?}", balance_new_partner1_after);
  println!("balance of fee partner receiver after {:?}", balance_new_fee_partner_receiver1_after);
  println!("balance of fee receiver after {:?}", balance_fee_receiver_after);

  //withdraw fungible token
  let balance_owner_before = token.balance(&mut app.wrap(), get_uaddr(OWNER)).unwrap();
  println!("balance of owner before {:?}", balance_owner_before);
  app.execute_contract(get_uaddr(OWNER), payment.clone(), &(ExecuteMsg::WithdrawFungibleToken {msg: WithdrawFungibleTokenMsg {
    token_addr: token.addr().into_string(),
    amount: Uint128::from(10u128),
    is_native_token: false
  }}),
  &[]
  ).unwrap();

  let balance_owner_after = token.balance(&mut app.wrap(), get_uaddr(OWNER)).unwrap();
  println!("balance of owner after {:?}", balance_owner_after);


  //Withdrawn nft
  let nft  = create_nft(&mut app ,get_uaddr(OWNER),Cw721ExecuteMsg{
    minter: OWNER.to_string(),
    name: "NFT".to_string(),
    symbol: "NFT".to_string(),
  }).unwrap();

  mint_nft(&mut app ,nft.clone(),get_uaddr(OWNER),payment.clone(),"1".to_string()).unwrap();
  let owner_before = query_owner(&mut app,&nft,"1".to_string());
  println!("owner nft before{:?}", owner_before);

  app.execute_contract(get_uaddr(OWNER), payment.clone(), &(ExecuteMsg::WithdrawNft {msg: WithdrawNftMsg {
    token_address: nft.clone(),
    token_ids: vec!["1".to_string()]
  }}),
  &[]
  ).unwrap();

  let owner_after = query_owner(&mut app,&nft,"1".to_string());
  println!("owner nft after{:?}", owner_after);

//Query get_partner_info 
let info_partner : PartnerInfoRespone = app.wrap()
      .query_wasm_smart(
          payment.clone(),
          &PaymentQueryMsg::GetPartnerInfo {msg: GetPartnerInfoMsg{
            partner_code: "1234".to_string()
          }
        }
      )
      .unwrap();

    println!("{:?}", info_partner);
}


}