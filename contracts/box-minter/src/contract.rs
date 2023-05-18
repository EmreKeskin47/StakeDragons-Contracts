use crate::msg::{
    ExecuteMsg, GetBoxResponse, GetStateResponse, InstantiateMsg, Metadata, MintBoxCrystal,
    QueryMsg, ReceiveMsg,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::WasmMsg::Execute;
use cosmwasm_std::{
    from_slice, to_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    Response, StdError, StdResult, SubMsg, Uint128,
};
use std::ops::Add;

use cw2::set_contract_version;
use cw20::{Cw20Contract, Cw20ExecuteMsg, Cw20ReceiveMsg};

pub type Cw721ExecuteMsg = cw721_base::ExecuteMsg<Metadata>;

use crate::error::ContractError;
use crate::helper::{generate_box_mint_msg, generate_crystal_mint_msg};
use crate::state::{ContractAddressList, State, BOX_COUNT, CONTRACTS, OPENED_BOX_COUNT, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:box-minter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: String::from(info.sender.clone()),
        base_price: msg.base_price,
        open_price: msg.open_price,
        random_key: msg.random_key,
        allowed_cw20: msg.allowed_cw20,
    };

    let contracts = ContractAddressList {
        dragon_box: "".to_string(),
        crystal: "".to_string(),
        multisig: msg.multisig,
        juno_recipient: msg.juno_recipient,
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;
    BOX_COUNT.save(deps.storage, &Uint128::new(0))?;
    OPENED_BOX_COUNT.save(deps.storage, &Uint128::new(0))?;
    CONTRACTS.save(deps.storage, &contracts)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::MintBox {} => execute_box_mint(deps, info),
        ExecuteMsg::GenesisMint {} => execute_genesis_box_mint(deps, info),
        ExecuteMsg::OpenBox(msg) => execute_free_hatch(deps, info, msg),
        ExecuteMsg::EditState {
            new_owner,
            base_price,
            random_key,
            open_price,
            allowed_cw20,
        } => execute_edit_state(
            deps,
            info,
            new_owner,
            base_price,
            random_key,
            allowed_cw20,
            open_price,
        ),
        ExecuteMsg::EditContracts {
            dragon_box,
            crystal,
            multisig,
            juno_recipient,
        } => execute_edit_contracts(deps, info, dragon_box, crystal, multisig, juno_recipient),
        ExecuteMsg::Receive(cw20_receive_msg) => {
            execute_receive(deps, _env, info, cw20_receive_msg)
        }
    }
}

pub fn execute_receive(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    wrapper: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let msg: ReceiveMsg = from_slice(&wrapper.msg)?;
    let amount = wrapper.amount;
    let sender = wrapper.sender;
    match msg {
        ReceiveMsg::Hatch { id, box_id } => {
            execute_open_cw20(deps, _env, info, sender, id, box_id, amount)
        }
    }
}

pub fn execute_open_cw20(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    sender: String,
    id: String,
    box_id: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let cfg = STATE.load(deps.storage)?;
    let contracts = CONTRACTS.load(deps.storage)?;

    if info.sender != cfg.allowed_cw20.clone() {
        return Err(ContractError::CW20TokenNotAllowed {
            sent: info.sender.to_string(),
            need: cfg.allowed_cw20.to_string(),
        });
    }

    let recipient_addr = deps.api.addr_validate(&contracts.multisig)?;

    //Send drgn to recipient dao address
    let cw20_execute_msg_fp = Cw20ExecuteMsg::Transfer {
        recipient: recipient_addr.into_string(),
        amount,
    };
    let fee_payout_msg = Cw20Contract(cfg.allowed_cw20)
        .call(cw20_execute_msg_fp)
        .map_err(ContractError::Std)?;

    //OPEN DRAGON BOX AND MINT CRYSTAL
    let crystal = contracts.crystal;
    let dragon_box = contracts.dragon_box;

    //id
    let id_type = id.clone();

    //crystal type
    let key1 = cfg.random_key / 10000;
    let key2 = cfg.random_key % 100;
    let secret = key1 + key2;
    let type_id = id_type.chars().nth(secret as usize).unwrap();

    let type_name;
    match type_id {
        'G' => type_name = "fire",
        '2' => type_name = "fire",
        'N' => type_name = "storm",
        '5' => type_name = "storm",
        'V' => type_name = "udin",
        'S' => type_name = "udin",
        '7' => type_name = "divine",
        '1' => type_name = "divine",
        _ => type_name = "ice",
    };

    let crystal_mint =
        generate_crystal_mint_msg(&*box_id.clone(), type_name.to_string(), sender.to_string())?;

    let crystal_mint_msg = CosmosMsg::Wasm(Execute {
        contract_addr: crystal,
        msg: to_binary(&crystal_mint)?,
        funds: vec![],
    });

    let burn_box = Cw721ExecuteMsg::Burn {
        token_id: box_id.clone(),
    };

    let burn_box_msg = CosmosMsg::Wasm(Execute {
        contract_addr: dragon_box,
        msg: to_binary(&burn_box)?,
        funds: vec![],
    });

    OPENED_BOX_COUNT.update::<_, StdError>(deps.storage, |id| Ok(id.add(Uint128::new(1))))?;

    let res = Response::new().add_submessages(vec![
        SubMsg::new(fee_payout_msg),
        SubMsg::new(crystal_mint_msg),
        SubMsg::new(burn_box_msg),
    ]);

    Ok(res)
}

pub fn execute_free_hatch(
    deps: DepsMut,
    info: MessageInfo,
    msg: MintBoxCrystal,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let contracts = CONTRACTS.load(deps.storage)?;

    if state.open_price != Uint128::new(0) && info.sender != state.owner {
        return Err(ContractError::Unauthorized {
            msg: "free hatch is no longer available".to_string(),
        });
    }
    let crystal = contracts.crystal;
    let dragon_box = contracts.dragon_box;

    //id
    let id_type = msg.clone().id;

    //crystal type
    let key1 = state.random_key / 10000;
    let key2 = state.random_key % 100;
    let secret = key1 + key2;
    let type_id = id_type.chars().nth(secret as usize).unwrap();

    let type_name;
    match type_id {
        'G' => type_name = "fire",
        '2' => type_name = "fire",
        'N' => type_name = "storm",
        '5' => type_name = "storm",
        'V' => type_name = "udin",
        'S' => type_name = "udin",
        '7' => type_name = "divine",
        '1' => type_name = "divine",
        _ => type_name = "ice",
    };

    let crystal_mint = generate_crystal_mint_msg(
        &*msg.clone().box_id.to_string(),
        type_name.to_string(),
        info.sender.to_string(),
    )?;

    let transfer_box = Cw721ExecuteMsg::Burn {
        token_id: String::from(msg.clone().box_id),
    };

    OPENED_BOX_COUNT.update::<_, StdError>(deps.storage, |id| Ok(id.add(Uint128::new(1))))?;

    Ok(Response::new()
        .add_message(CosmosMsg::Wasm(Execute {
            contract_addr: dragon_box,
            msg: to_binary(&transfer_box)?,
            funds: vec![],
        }))
        .add_message(CosmosMsg::Wasm(Execute {
            contract_addr: crystal,
            msg: to_binary(&crystal_mint)?,
            funds: vec![],
        }))
        .add_attribute("box burn", msg.id)
        .add_attribute("mint crystal for", info.sender))
}

/// box mint
pub fn execute_box_mint(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let contracts = CONTRACTS.load(deps.storage)?;

    if info.funds.len() != 1 {
        return Err(ContractError::SendSingleNativeToken {});
    }

    let sent_fund = info.funds.get(0).unwrap();
    //Main => ujuno
    if sent_fund.denom != "ujuno" {
        return Err(ContractError::NativeDenomNotAllowed {
            denom: sent_fund.clone().denom,
        });
    }

    let fee = Uint128::from(state.base_price);

    // check price matches
    if fee != sent_fund.amount {
        return Err(ContractError::SentWrongFundsAmount {
            need: fee,
            sent: sent_fund.amount,
        });
    }

    let payment = BankMsg::Send {
        to_address: contracts.juno_recipient.clone(),
        amount: vec![Coin {
            denom: "ujuno".to_string(),
            amount: fee,
        }],
    };

    let boxes = BOX_COUNT.update::<_, StdError>(deps.storage, |id| Ok(id.add(Uint128::new(1))))?;
    let id = "00000".to_string() + &*boxes.to_string();

    let msg = generate_box_mint_msg(&*id.to_string(), info.clone().sender.to_string())?;
    Ok(Response::new()
        .add_message(CosmosMsg::Wasm(Execute {
            contract_addr: contracts.dragon_box.clone(),
            msg: to_binary(&(msg))?,
            funds: vec![],
        }))
        .add_messages(vec![payment]))
}

/// box mint free
pub fn execute_genesis_box_mint(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let contracts = CONTRACTS.load(deps.storage)?;
    let fee = Uint128::from(state.base_price);

    if fee != Uint128::new(0) && info.sender != state.owner {
        return Err(ContractError::Unauthorized {
            msg: "genesis sale has ended".to_string(),
        });
    }

    let boxes = BOX_COUNT.update::<_, StdError>(deps.storage, |id| Ok(id.add(Uint128::new(1))))?;
    let id = "00000".to_string() + &*boxes.to_string();

    let msg = generate_box_mint_msg(&*id.to_string(), info.clone().sender.to_string())?;
    Ok(Response::new().add_message(CosmosMsg::Wasm(Execute {
        contract_addr: contracts.dragon_box.clone(),
        msg: to_binary(&(msg))?,
        funds: vec![],
    })))
}

pub fn execute_edit_state(
    deps: DepsMut,
    info: MessageInfo,
    new_owner: String,
    base_price: Uint128,
    random_key: i32,
    allowed_cw20: Addr,
    open_price: Uint128,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {
            msg: "state can only be edited by the owner".to_string(),
        });
    }
    let new = State {
        owner: new_owner,
        base_price,
        random_key,
        allowed_cw20,
        open_price,
    };
    STATE.save(deps.storage, &new)?;
    Ok(Response::new())
}

pub fn execute_edit_contracts(
    deps: DepsMut,
    info: MessageInfo,
    dragon_box: String,
    crystal: String,
    multisig: String,
    juno_recipient: String,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {
            msg: "state can only be edited by the owner".to_string(),
        });
    }
    let new = ContractAddressList {
        dragon_box,
        crystal,
        multisig,
        juno_recipient,
    };

    CONTRACTS.save(deps.storage, &new)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetState {} => to_binary(&query_state(deps)?),
        QueryMsg::GetBoxListInfo {} => to_binary(&query_box_list(deps)?),
    }
}

fn query_state(deps: Deps) -> StdResult<GetStateResponse> {
    let state = STATE.load(deps.storage)?;
    let contract = CONTRACTS.load(deps.storage)?;
    Ok(GetStateResponse {
        owner: state.owner,
        base_price: state.base_price,
        open_price: state.open_price,
        random_key: state.random_key,
        allowed_cw20: state.allowed_cw20,
        dragon_box: contract.dragon_box,
        crystal: contract.crystal,
        multisig: contract.multisig,
        juno_recipient: contract.juno_recipient,
    })
}

fn query_box_list(deps: Deps) -> StdResult<GetBoxResponse> {
    let state = STATE.load(deps.storage)?;
    let opened = OPENED_BOX_COUNT.load(deps.storage)?;
    let total = BOX_COUNT.load(deps.storage)?;

    Ok(GetBoxResponse {
        opened,
        total,
        base_price: state.base_price,
        open_price: state.open_price,
    })
}
