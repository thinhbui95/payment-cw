use cosmwasm_std::{Addr,Empty,Uint128,StdError,Coin, coins};
use cw20_base::msg::InstantiateMsg as Cw20InstantiateMsg;
use cw_multi_test::{ App, Contract, ContractWrapper, Executor,AppResponse};
use cw20::Cw20Coin;
use crate::error::*;
use cw_multi_test::{BankSudo, SudoMsg as CwSudoMsg};
use cw721_base::{InstantiateMsg, ExecuteMsg as Cw721ExecuteMsg,QueryMsg};
use cw721::OwnerOfResponse;


pub fn get_uaddr(addr: &str) -> Addr {
    let addr = Addr::unchecked(addr);
    addr

}

pub fn cw20() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(cw20_base::contract::execute, cw20_base::contract::instantiate, cw20_base::contract::query);
    Box::new(contract)
}

pub fn cw721() -> Box<dyn Contract<Empty>> {
  let contract = ContractWrapper::new(cw721_base::entry::execute, cw721_base::entry::instantiate, cw721_base::entry::query);
  Box::new(contract)
}


pub fn create_cw20_token(app: &mut App, minter: &Addr, users: Vec<Addr>) -> Addr {
  let id = app.store_code(cw20());
  app
    .instantiate_contract(
      id,
      minter.clone(),
      &(Cw20InstantiateMsg {
        decimals: 18,
        initial_balances: vec![
          Cw20Coin {
            address: String::from(users[0].clone()),
            amount: (1000000u128).into(),
          },
          Cw20Coin {
            address: String::from(users[1].clone()),
            amount: (1000000u128).into(),
          },
          Cw20Coin {
            address: String::from(users[2].clone()),
            amount: (1000000u128).into(),
          },
        ],
        name: String::from("TestCw20"),
        symbol: String::from("TEST"),
        mint: Some(cw20::MinterResponse { minter: minter.to_string(), cap: None }),
        marketing: None,
      }),
      &[],
      "Token",
      None
    )
    .unwrap()
}

pub fn approve_cw20_token(
    app: &mut App,
    owner: &Addr,
    spender: Addr,
    contract_addr: &Addr,
    amount: Uint128
  ) -> Result<AppResponse, StdError> {
    let msg = cw20::Cw20ExecuteMsg::IncreaseAllowance {
      spender: spender.to_string(),
      amount,
      expires: None,
    };

    app.store_code(cw20());
    let result = app
      .execute_contract(owner.clone(), contract_addr.clone(), &msg, &[])
      .unwrap();

    Ok(result)
  }


pub fn mint_native_token(
    app: &mut App,
    receiver: &Addr,
    amount: u128,
    denom: &str
  ) -> Result<Vec<Coin>, ContractError> {
    let fee_funds = coins(amount, denom);
    app
      .sudo(
        CwSudoMsg::Bank({ BankSudo::Mint {
            to_address: receiver.to_string(),
            amount: fee_funds.clone(),
          } })
      )
      .map_err(|err| println!("{:?}", err))
      .ok();

    Ok(fee_funds)
}

pub fn create_nft(app: &mut App,sender: Addr,instantiate_msg: InstantiateMsg) -> Result<Addr, StdError> {
  let id = app.store_code(cw721());
  let contract = app
  .instantiate_contract(
    id,
    sender,
    &instantiate_msg,
    &[],
    "collection-manage",
    None
  )
  .unwrap();

  Ok(contract)
}

pub fn mint_nft(app: &mut App, cw721: Addr, sender: Addr,receiver: Addr, token_id: String) -> Result<AppResponse, StdError>{
  let result = app.execute_contract(
      sender.clone(),
      cw721.clone(),
      &Cw721ExecuteMsg::<Empty, Empty>::Mint {
          token_id: token_id.clone(),
          owner: receiver.to_string(),
          token_uri: None,
          extension: Empty::default(),
      },
      &[],
  )
  .unwrap();

  Ok(result)
}

pub fn transfer_nft(app: &mut App, cw721: Addr, sender: Addr,receiver: Addr, token_id: String) -> Result<AppResponse, StdError>{
  let result = app.execute_contract(
      sender.clone(),
      cw721.clone(),
      &Cw721ExecuteMsg::<Empty, Empty>::TransferNft {
        recipient: receiver.to_string(),
        token_id: token_id.clone(),
      },
      &[],
  )
  .unwrap();

  Ok(result)
}


pub fn query_owner(app: &mut App , cw721: &Addr, token_id: String) -> Addr {
  let resp: OwnerOfResponse = app.wrap()
      .query_wasm_smart(
          cw721,
          &QueryMsg::<Empty>::OwnerOf {
              token_id,
              include_expired: None,
          },
      )
      .unwrap();
  Addr::unchecked(resp.owner)
}





