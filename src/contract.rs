use cosmwasm_std::{entry_point, CosmosMsg, DepsMut, Empty, Env, MessageInfo, Response, StdError};

use crate::msg::ExecuteMsg;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> Result<Response, StdError> {
    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {

    let msgs: Vec<CosmosMsg> = msg.msgs
        .into_iter()
        .filter(|msg| match msg {
            CosmosMsg::Stargate { .. } => true,
            _ => false,
        })
        .collect();
    Ok(Response::new().add_messages(msgs))
    
}