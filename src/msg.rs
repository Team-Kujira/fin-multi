use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use kujira::denom::Denom;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub struct ExecuteMsg {
    pub stages: Vec<Vec<(Addr, Denom)>>,
    pub recipient: Option<Addr>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
