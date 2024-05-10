#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, ensure, wasm_execute, Addr, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Empty, Env,
    MessageInfo, QuerierWrapper, Response, StdError, StdResult,
};
use kujira::Denom;

use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, CALLBACK_ADDRESS};

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
    let storage = deps.storage;
    let querier = deps.querier;
    let mut stages = msg.stages;
    match stages.pop() {
        None => {
            // We're done, return balances to sender
            let balances = querier.query_all_balances(env.contract.address)?;
            if let Some(min_return) = msg.min_return {
                for min in min_return {
                    let denom = balances.iter().find(|x| x.denom == min.denom);
                    ensure!(
                        denom.map_or(false, |x| x.amount >= min.amount),
                        StdError::generic_err(format!("insufficient return amount {}", min.denom))
                    );
                }
            }

            let return_msg = match msg.callback {
                Some(callback) => {
                    let callback_address = if let Some(addr) = CALLBACK_ADDRESS.may_load(storage)? {
                        CALLBACK_ADDRESS.remove(storage);
                        addr
                    } else {
                        info.sender.clone()
                    };
                    callback.to_message(&callback_address, Empty {}, balances)?
                }
                None => BankMsg::Send {
                    to_address: msg
                        .recipient
                        .ok_or_else(|| StdError::generic_err("recipient not set"))?
                        .to_string(),
                    amount: balances,
                }
                .into(),
            };

            Ok(Response::default().add_message(return_msg))
        }
        Some(s) => {
            let msgs = execute_swaps(querier, &env, s)?;
            // store info.sender on first call if callback is set
            if msg.callback.is_some() {
                let callback_address = CALLBACK_ADDRESS.may_load(storage)?;
                if callback_address.is_none() {
                    CALLBACK_ADDRESS.save(storage, &info.sender)?;
                }
            }
            Ok(Response::default()
                .add_messages(msgs)
                .add_message(wasm_execute(
                    env.contract.address,
                    &ExecuteMsg {
                        stages,
                        // If this is the first call, and the sender has explicity set a recipient, make sure that
                        // the sender is loaded for future calls
                        recipient: Some(msg.recipient.unwrap_or(info.sender.clone())),
                        min_return: msg.min_return,
                        callback: msg.callback,
                    },
                    vec![],
                )?))
        }
    }
}

fn execute_swaps(
    querier: QuerierWrapper,
    env: &Env,
    addrs: Vec<(Addr, Denom)>,
) -> StdResult<Vec<CosmosMsg>> {
    let balances = querier.query_all_balances(&env.contract.address)?;
    let mut msgs: Vec<CosmosMsg> = vec![];
    for (addr, denom) in addrs {
        let balance = balances.iter().find(|b| b.denom == denom.to_string());
        if let Some(coin) = balance {
            let msg = wasm_execute(
                addr,
                &kujira::fin::ExecuteMsg::Swap {
                    offer_asset: None,
                    belief_price: None,
                    max_spread: None,
                    to: None,
                    callback: None,
                },
                coins(coin.amount.u128(), coin.denom.clone()),
            )?;

            msgs.push(msg.into());
        }
    }
    Ok(msgs)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg(test)]
mod tests {}
