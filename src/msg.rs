use cosmwasm_schema::cw_serde;
use cosmwasm_std::CosmosMsg;

#[cw_serde]
pub struct ExecuteMsg {
    pub msgs: Vec<CosmosMsg>,
}