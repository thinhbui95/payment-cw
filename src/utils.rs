
use cosmwasm_std::{SubMsg, coins,BankMsg,WasmMsg,Response, to_json_binary, Event, DepsMut, ensure , Attribute,MessageInfo,Uint128,Addr};
use cw20::{Cw20ExecuteMsg, Cw20Coin};
use cw721::Cw721ExecuteMsg;
use crate::{error::ContractError, state::*};

//Transfer coin from user to contract
pub fn transfer_from_coin(info: MessageInfo, token: Cw20Coin,  contract_addr: Addr, is_native_token: bool, res: &mut Response) -> Result<(), ContractError> {
    if token.amount <= Uint128::zero() {
      return Err(ContractError::InvalidAmount{ });
    }
    if is_native_token {
      let funds = info.funds;
      let funds_index = funds.iter().enumerate().find(|f| f.1.denom == token.address).map(|t| t.0).unwrap();
      ensure!(funds[funds_index].amount >= token.amount, ContractError::InvalidAmount{ });
    } else {
      let transfer_msg = Cw20ExecuteMsg::TransferFrom {
        owner: info.sender.to_string(),
        recipient: contract_addr.to_string(),
        amount: token.amount,
      };

      res.messages.push(SubMsg::new(WasmMsg::Execute {
        contract_addr: token.address,
        msg: to_json_binary(&transfer_msg)?,
        funds: vec![],
      }));
    }

    Ok(())
  }


// Transfer token from contract to receiver 
pub fn transfer_coin(_info: MessageInfo, token: Cw20Coin, to: Addr,is_native_token:bool,res: &mut Response) -> Result<(), ContractError> {
    if token.amount <= Uint128::zero() {
        return Err(ContractError::InvalidAmount{ });
    }
    if is_native_token {
        res.messages.push(SubMsg::new(BankMsg::Send {
          to_address: to.to_string(),
          amount: coins(token.amount.u128(),
          token.address)
        }))
      } else {
        let transfer_msg = Cw20ExecuteMsg::Transfer {
            recipient: to.to_string(),
            amount: token.amount,
        };

        res.messages.push(SubMsg::new(WasmMsg::Execute {
            contract_addr: token.address,
            msg: to_json_binary(&transfer_msg)?,
            funds: vec![],
        }));
    }
    Ok(())
}

pub fn transfer_nft(token:Addr, token_id: String , to: Addr, res: &mut Response) -> Result<(), ContractError> {
    let cw721_transfer_msg = Cw721ExecuteMsg::TransferNft {
      recipient: to.to_string(),
      token_id: token_id.to_string(),
    };

    res.messages.push(SubMsg::new(WasmMsg::Execute {
      contract_addr: token.to_string(),
      msg: to_json_binary(&cw721_transfer_msg)?,
      funds: vec![],
    }));

    Ok(())
  }


pub fn _set_admin(admin: Addr, is_active: &bool,deps: DepsMut,res: &mut Response) -> Result<(), ContractError> {
    IS_ACTIVED.save(deps.storage, admin.clone(), is_active)?;
    let event = Event::new("admins_updated").add_attribute(admin.to_string(), is_active.to_string());
    res.events.push(event);
    Ok(())
}

pub fn _set_admins(admins: Vec<Addr>, is_actives: Vec<bool>,deps: DepsMut,res: &mut Response) -> Result<(), ContractError> {
    let mut attrs: Vec<Attribute> = vec![];
    for i in 0..admins.len() {
        IS_ACTIVED.save(deps.storage, admins[i].clone(), &is_actives[i])?;
        attrs.push( Attribute{ key : admins[i].to_string(), value: is_actives[i].to_string()});
    }
    let event = Event::new("admins_updated").add_attributes(attrs);
    res.events.push(event);
    Ok(())
}

pub fn option_bytes_to_string(bytes_option: Option<[u8; 32]>) -> String {
  bytes_option
      .map(|bytes| bytes.iter().map(|&b| format!("{:02X}", b)).collect::<String>())
      .unwrap_or_else(|| "Empty data".to_string())
}









  