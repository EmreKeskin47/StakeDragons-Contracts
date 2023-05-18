use crate::ContractError::Unauthorized;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::WasmMsg::Execute;
use cosmwasm_std::{
    from_slice, to_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    Response, StdError, StdResult, SubMsg, Uint128, Uint64,
};
use cw2::set_contract_version;
use cw20::{Cw20Contract, Cw20ExecuteMsg, Cw20ReceiveMsg};
use std::ops::Add;

pub type Cw721ExecuteMsg = cw721_base::ExecuteMsg<Metadata>;

use crate::error::ContractError;
use crate::helper::{generate_dragon_mint_msg, generate_egg_mint_msg};
use crate::msg::{
    ExecuteMsg, GetEggsaleInfoResponse, GetStateResponse, InstantiateMsg, Metadata, MintEggDragon,
    QueryMsg, ReceiveMsg,
};
use crate::state::{ContractAddressList, State, CONTRACTS, EGG_SALE_COUNT, STATE, TOTAL_EGGS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:minter";
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
        hatch_price: msg.hatch_price,
        random_key: msg.random_key,
        egg_sale_size: msg.egg_sale_size,
        allowed_cw20: msg.allowed_cw20,
    };

    let contracts = ContractAddressList {
        egg: "".to_string(),
        dragon: "".to_string(),
        recipient: "juno1luw9kspq5dwrarxgkvfwt443ue99umdaqmm57afg7ms5yOrkz3dsfeudxp".to_string(),
        multisig: "juno1luw9kspq5dwrqrxgkvfwt443ue99umdaqmm57afg7ms5y0rkz3dsfeudxp".to_string(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;
    EGG_SALE_COUNT.save(deps.storage, &Uint64::new(0))?;
    TOTAL_EGGS.save(deps.storage, &Uint64::new(0))?;
    CONTRACTS.save(deps.storage, &contracts)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::MintEgg {} => execute_egg_mint(deps, info),
        ExecuteMsg::GenesisHatch(msg) => execute_free_hatch(deps, info, msg),
        ExecuteMsg::DragonBirth { id, owner } => execute_dragon_birth(deps, info, id, owner),
        ExecuteMsg::EditState {
            new_owner,
            base_price,
            random_key,
            hatch_price,
            egg_sale_size,
            allowed_cw20,
        } => execute_edit_state(
            deps,
            info,
            new_owner,
            base_price,
            random_key,
            hatch_price,
            egg_sale_size,
            allowed_cw20,
        ),
        ExecuteMsg::EditContracts {
            egg,
            dragon,
            recipient,
            multisig,
        } => execute_edit_contracts(deps, info, egg, dragon, recipient, multisig),
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
        ReceiveMsg::Hatch { id, egg_id } => {
            execute_hatch_cw20(deps, _env, info, sender, id, egg_id, amount)
        }
    }
}

pub fn execute_hatch_cw20(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    sender: String,
    id: String,
    egg_id: String,
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

    let recipient_addr = deps.api.addr_validate(&contracts.recipient)?;

    //Send drgn to recipient dao address
    let cw20_execute_msg_fp = Cw20ExecuteMsg::Transfer {
        recipient: recipient_addr.into_string(),
        amount,
    };
    let fee_payout_msg = Cw20Contract(cfg.allowed_cw20)
        .call(cw20_execute_msg_fp)
        .map_err(ContractError::Std)?;

    //HATCH
    let dragon = contracts.dragon;
    let egg = contracts.egg;

    //id
    let id_type = id.clone();

    //dragon type
    let key1 = cfg.random_key / 10000;
    let key2 = cfg.random_key % 100;
    let secret = key1 + key2;
    let type_id = id_type.chars().nth(secret as usize).unwrap();

    let type_name;
    match type_id {
        '2' => type_name = "uncommon",
        '3' => type_name = "rare",
        '4' => type_name = "epic",
        '5' => type_name = "legendary",
        'P' => type_name = "legendary",
        'K' => type_name = "epic",
        'G' => type_name = "rare",
        'W' => type_name = "uncommon",
        _ => type_name = "common",
    };

    let dragon_mint =
        generate_dragon_mint_msg(&*egg_id.clone(), type_name.to_string(), sender.to_string())?;

    let dragon_mint_msg = CosmosMsg::Wasm(Execute {
        contract_addr: dragon,
        msg: to_binary(&dragon_mint)?,
        funds: vec![],
    });

    let burn_egg = Cw721ExecuteMsg::Burn {
        token_id: egg_id.clone(),
    };

    let burn_egg_msg = CosmosMsg::Wasm(Execute {
        contract_addr: egg,
        msg: to_binary(&burn_egg)?,
        funds: vec![],
    });

    let res = Response::new()
        .add_submessages(vec![
            SubMsg::new(fee_payout_msg),
            SubMsg::new(dragon_mint_msg),
            SubMsg::new(burn_egg_msg),
        ])
        .add_attribute("id", id)
        .add_attribute("egg_id ", egg_id)
        .add_attribute("action", "buy_cw20")
        .add_attribute("fee", amount);

    Ok(res)
}

pub fn execute_free_hatch(
    deps: DepsMut,
    info: MessageInfo,
    msg: MintEggDragon,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let contracts = CONTRACTS.load(deps.storage)?;

    if state.hatch_price != Uint128::new(0) && info.sender != state.owner {
        return Err(ContractError::Unauthorized {
            msg: "free hatch is no longer available".to_string(),
        });
    }
    let dragon = contracts.dragon;
    let egg = contracts.egg;

    //id
    let id_type = msg.clone().id;

    //dragon type
    let key1 = state.random_key / 10000;
    let key2 = state.random_key % 100;
    let secret = key1 + key2;
    let type_id = id_type.chars().nth(secret as usize).unwrap();

    let type_name;
    match type_id {
        '2' => type_name = "uncommon",
        '3' => type_name = "rare",
        '4' => type_name = "epic",
        '5' => type_name = "legendary",
        'P' => type_name = "legendary",
        'K' => type_name = "epic",
        'G' => type_name = "rare",
        'W' => type_name = "uncommon",
        _ => type_name = "common",
    };

    let dragon_mint = generate_dragon_mint_msg(
        &*msg.clone().egg_id.to_string(),
        type_name.to_string(),
        info.sender.to_string(),
    )?;

    let transfer_egg = Cw721ExecuteMsg::Burn {
        token_id: String::from(msg.clone().egg_id),
    };

    Ok(Response::new()
        .add_message(CosmosMsg::Wasm(Execute {
            contract_addr: egg,
            msg: to_binary(&transfer_egg)?,
            funds: vec![],
        }))
        .add_message(CosmosMsg::Wasm(Execute {
            contract_addr: dragon,
            msg: to_binary(&dragon_mint)?,
            funds: vec![],
        }))
        .add_attribute("egg burn", msg.id)
        .add_attribute("mint dragon for", info.sender))
}

/// Egg mint for eggsale
pub fn execute_egg_mint(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let contracts = CONTRACTS.load(deps.storage)?;
    let count = EGG_SALE_COUNT.load(deps.storage)?;

    if count >= state.egg_sale_size {
        return Err(ContractError::EggSaleLimit {});
    }

    if info.funds.len() != 1 {
        return Err(ContractError::SendSingleNativeToken {});
    }

    let sent_fund = info.funds.get(0).unwrap();
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
        to_address: contracts.multisig.clone(),
        amount: vec![Coin {
            denom: "ujuno".to_string(),
            amount: fee,
        }],
    };

    EGG_SALE_COUNT.update::<_, StdError>(deps.storage, |id| Ok(id.add(Uint64::new(1))))?;
    let eggs = TOTAL_EGGS.update::<_, StdError>(deps.storage, |id| Ok(id.add(Uint64::new(1))))?;
    let id = "00000".to_string() + &*eggs.to_string();

    let msg = generate_egg_mint_msg(&*id.to_string(), info.clone().sender.to_string())?;
    Ok(Response::new()
        .add_message(CosmosMsg::Wasm(Execute {
            contract_addr: contracts.egg.clone(),
            msg: to_binary(&(msg))?,
            funds: vec![],
        }))
        .add_messages(vec![payment]))
}

/// Egg mint free
pub fn execute_dragon_birth(
    deps: DepsMut,
    info: MessageInfo,
    _id: String,
    owner: String,
) -> Result<Response, ContractError> {
    let contracts = CONTRACTS.load(deps.storage)?;
    let state = STATE.load(deps.storage)?;

    if info.sender != contracts.dragon && info.sender != state.owner {
        return Err(Unauthorized {
            msg: "only dragon contract can execute dragon birth ".to_string(),
        });
    }

    let total = TOTAL_EGGS.update::<_, StdError>(deps.storage, |id| Ok(id.add(Uint64::new(1))))?;
    let egg_id = "00000".to_string() + &*total.to_string();

    let msg = generate_egg_mint_msg(&*egg_id.to_string(), owner)?;
    Ok(Response::new().add_message(CosmosMsg::Wasm(Execute {
        contract_addr: contracts.egg,
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
    hatch_price: Uint128,
    egg_sale_size: Uint64,
    allowed_cw20: Addr,
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
        hatch_price,
        random_key,
        egg_sale_size,
        allowed_cw20,
    };
    STATE.save(deps.storage, &new)?;
    Ok(Response::new())
}

pub fn execute_edit_contracts(
    deps: DepsMut,
    info: MessageInfo,
    egg: String,
    dragon: String,
    recipient: String,
    multisig: String,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {
            msg: "state can only be edited by the owner".to_string(),
        });
    }
    let new = ContractAddressList {
        egg,
        dragon,
        recipient,
        multisig,
    };

    CONTRACTS.save(deps.storage, &new)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetState {} => to_binary(&query_state(deps)?),
        QueryMsg::GetEggsaleOwnedCount {} => to_binary(&query_eggsale(deps)?),
    }
}

fn query_state(deps: Deps) -> StdResult<GetStateResponse> {
    let state = STATE.load(deps.storage)?;
    let contract = CONTRACTS.load(deps.storage)?;
    let total_eggs = TOTAL_EGGS.load(deps.storage)?;
    Ok(GetStateResponse {
        owner: state.owner,
        base_price: state.base_price,
        random_key: state.random_key,
        hatch_price: state.hatch_price,
        total_eggs,
        egg_sale_size: state.egg_sale_size,
        egg_contract: contract.egg,
        dragon_contract: contract.dragon,
        recipient_contract: contract.recipient,
        multisig_contract: contract.multisig,
    })
}

fn query_eggsale(deps: Deps) -> StdResult<GetEggsaleInfoResponse> {
    let state = STATE.load(deps.storage)?;
    let owned = EGG_SALE_COUNT.load(deps.storage)?;

    Ok(GetEggsaleInfoResponse {
        owned_eggsale: owned,
        size: state.egg_sale_size,
        base_price: state.base_price,
    })
}
