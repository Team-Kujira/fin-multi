#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, to_binary, Addr, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response,
    StdError, StdResult, WasmMsg,
};
use kujira::denom::Denom;

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    let mut stages = msg.stages;
    match stages.pop() {
        None => {
            // We're done, return balances to sender
            let balances = deps.querier.query_all_balances(env.contract.address)?;
            if let Some(min_return) = msg.min_return {
                for min in min_return {
                    let err = Err(StdError::generic_err(format!(
                        "insufficient return amount {}",
                        min.denom
                    )));
                    match balances.iter().find(|x| x.denom == min.denom) {
                        None => return err,
                        Some(balance) => {
                            if balance.amount < min.amount {
                                return err;
                            }
                        }
                    }
                }
            }

            Ok(
                Response::default().add_message(CosmosMsg::Bank(BankMsg::Send {
                    to_address: msg
                        .recipient
                        .ok_or_else(|| StdError::generic_err("recipient not set"))?
                        .to_string(),
                    amount: balances,
                })),
            )
        }
        Some(s) => {
            let contract_addr = env.contract.address.to_string();
            let msgs = execute_swaps(deps, env, s)?;
            Ok(Response::default()
                .add_messages(msgs)
                .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr,
                    msg: to_binary(&ExecuteMsg {
                        stages,
                        // If this is the first call, and the sender has explicity set a recipient, make sure that
                        // the sender is loaded for future calls
                        recipient: Some(msg.recipient.unwrap_or(info.sender)),
                        min_return: msg.min_return,
                    })?,
                    funds: vec![],
                })))
        }
    }
}

fn execute_swaps(deps: DepsMut, env: Env, addrs: Vec<(Addr, Denom)>) -> StdResult<Vec<CosmosMsg>> {
    let balances = deps.querier.query_all_balances(env.contract.address)?;
    let mut msgs: Vec<CosmosMsg> = vec![];
    for (addr, denom) in addrs {
        let balance = balances.iter().find(|b| b.denom == denom.to_string());
        if let Some(coin) = balance {
            let msg = CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: addr.to_string(),
                msg: to_binary(&kujira::fin::ExecuteMsg::Swap {
                    offer_asset: None,
                    belief_price: None,
                    max_spread: None,
                    to: None,
                })
                .unwrap(),
                funds: coins(coin.amount.u128(), coin.denom.clone()),
            });

            msgs.push(msg);
        }
    }
    Ok(msgs)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
