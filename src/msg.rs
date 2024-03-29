use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;
use kujira::{CallbackData, Denom};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub struct ExecuteMsg {
    pub stages: Vec<Vec<(Addr, Denom)>>,
    pub recipient: Option<Addr>,
    pub min_return: Option<Vec<Coin>>,
    /// An optional callback that FIN Multi will execute with the funds from the swap.
    /// The callback is executed on the sender's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<CallbackData>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

#[cw_serde]
pub struct MigrateMsg {}

pub const CALLBACK_ADDRESS: Item<Addr> = Item::new("callback_address");
