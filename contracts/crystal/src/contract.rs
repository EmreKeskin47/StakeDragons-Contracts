use crate::error::ContractError;
use crate::msg::{
    CollectionInfoResponse, CustomMintMsg, ExecuteMsg, Extension, InstantiateMsg, QueryMsg,
    ReceiveMsg, StateResponse,
};
use crate::state::{
    CollectionInfo, Crystal, CrystalListResponse, CrystalResponse, State, COLLECTION_INFO,
    COSMIC_LENGTH, CRYSTAL_INFO, CRYSTAL_INFO_SEQ, STATE,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
// use cosmwasm_std::WasmMsg::Execute;
use cosmwasm_std::WasmMsg::Execute;
use cosmwasm_std::{
    from_slice, to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo, Order,
    Response, StdError, StdResult, SubMsg, Uint128, Uint64,
};
use cw2::set_contract_version;
use cw20::{Cw20Contract, Cw20ExecuteMsg, Cw20ReceiveMsg};
use std::ops::Add;

pub type Cw721Contract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty>;
pub type Cw721ExecuteMsg = cw721_base::ExecuteMsg<Extension>;
// use crate::helper::generate_cosmic_mint_msg;
use crate::helper::generate_cosmic_mint_msg;
use cw_storage_plus::Bound;
use cw_utils::Expiration;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:dragon-mint";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let minter = deps.api.addr_validate(&msg.base.minter)?;
    Cw721Contract::default().instantiate(deps.branch(), env, info.clone(), msg.base.clone())?;

    let state = State {
        owner: String::from(info.sender.clone()),
        cosmic_contract: msg.cosmic_contract,
        drgn_recipient: msg.drgn_recipient,
        allowed_cw20: msg.allowed_cw20,
        attune_price: msg.attune_price,
    };

    let collection_info = CollectionInfo {
        name: msg.base.name,
        symbol: msg.base.symbol,
        minter: minter.to_string(),
        size: msg.size,
    };

    STATE.save(deps.storage, &state)?;
    COLLECTION_INFO.save(deps.storage, &collection_info)?;
    CRYSTAL_INFO_SEQ.save(deps.storage, &Uint64::zero())?;
    COSMIC_LENGTH.save(deps.storage, &Uint64::zero())?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateOwner { new_owner } => execute_update_owner(deps, info, new_owner),
        ExecuteMsg::UpdateState {
            cosmic_contract,
            drgn_recipient,
            allowed_cw20,
            attune_price,
        } => execute_update_state(
            deps,
            info,
            cosmic_contract,
            drgn_recipient,
            allowed_cw20,
            attune_price,
        ),

        ExecuteMsg::Mint(msg) => execute_mint(deps, env, info, msg),
        ExecuteMsg::Burn { token_id } => execute_burn(deps, env, info, token_id),
        ExecuteMsg::TransferNft {
            recipient,
            token_id,
        } => execute_transfer_nft(deps, env, info, recipient, token_id),
        ExecuteMsg::Approve {
            spender,
            token_id,
            expires,
        } => execute_approve(deps, env, info, spender, token_id, expires),
        ExecuteMsg::ApproveAll { operator, expires } => {
            execute_approve_all(deps, env, info, operator, expires)
        }
        ExecuteMsg::SendNft {
            contract,
            token_id,
            msg,
        } => execute_send_nft(deps, env, info, contract, token_id, msg),
        ExecuteMsg::Revoke { spender, token_id } => {
            execute_revoke(deps, env, info, spender, token_id)
        }
        ExecuteMsg::RevokeAll { operator } => execute_revoke_all(deps, env, info, operator),
        ExecuteMsg::GenerateCosmic {
            fire_id,
            ice_id,
            storm_id,
            divine_id,
            udin_id,
        } => execute_generate_cosmic(
            deps, info, env, fire_id, ice_id, storm_id, divine_id, udin_id,
        ),
        ExecuteMsg::Receive(cw20_receive_msg) => execute_receive(deps, env, info, cw20_receive_msg),
    }
}

pub fn execute_receive(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    wrapper: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let msg: ReceiveMsg = from_slice(&wrapper.msg)?;
    let amount = wrapper.amount;
    let sender = wrapper.sender;
    match msg {
        ReceiveMsg::GenerateCosmic {
            fire_id,
            ice_id,
            storm_id,
            divine_id,
            udin_id,
            owner
        } => execute_open_cw20(
            deps, env, info, amount, sender, fire_id, ice_id, storm_id, divine_id, udin_id,
            owner,
        ),
    }
}

pub fn execute_open_cw20(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
    _sender: String,
    fire_id: Uint64,
    ice_id: Uint64,
    storm_id: Uint64,
    divine_id: Uint64,
    udin_id: Uint64,
    owner: String,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;

    if info.sender != state.allowed_cw20.clone() {
        return Err(ContractError::CW20TokenNotAllowed {
            sent: info.sender.to_string(),
            need: state.allowed_cw20.to_string(),
        });
    }

    // check price matches
    if state.attune_price != amount {
        return Err(ContractError::SentWrongFundsAmount {
            need: state.attune_price,
            sent: amount,
        });
    }

    //Send drgn to recipient dao address
    let cw20_execute_msg_fp = Cw20ExecuteMsg::Transfer {
        recipient: state.drgn_recipient,
        amount,
    };
    let fee_payout_msg = Cw20Contract(state.allowed_cw20)
        .call(cw20_execute_msg_fp)
        .map_err(ContractError::Std)?;

    //Check for different types exist
    let (fire, ice, storm, divine, udin) = (
        CRYSTAL_INFO.load(deps.storage, fire_id.u64())?,
        CRYSTAL_INFO.load(deps.storage, ice_id.u64())?,
        CRYSTAL_INFO.load(deps.storage, storm_id.u64())?,
        CRYSTAL_INFO.load(deps.storage, divine_id.u64())?,
        CRYSTAL_INFO.load(deps.storage, udin_id.u64())?,
    );
    let crystals = [fire, ice, storm, divine, udin];
    let kinds = ["fire", "ice", "storm", "divine", "udin"];
    for index in 0..5 {
        if crystals[index].owner != owner {
            return Err(ContractError::Unauthorized {});
        } else if crystals[index].kind != kinds[index] {
            return Err(ContractError::KindNotFound {});
        }
    }
    for index in 0..5 {
        let msg = Cw721ExecuteMsg::Burn {
            token_id: crystals[index].clone().token_id,
        };
        Cw721Contract::default()
            .execute(deps.branch(), env.clone(), info.clone(), msg)
            .unwrap();
    }
    //Delete from the local list
    for index in 0..5 {
        let mut crystal = CRYSTAL_INFO.load(
            deps.storage,
            crystals[index].clone().token_id.parse::<u64>().unwrap(),
        )?;
        crystal.owner = "".to_string();
        CRYSTAL_INFO.save(
            deps.storage,
            crystals[index].clone().token_id.parse::<u64>().unwrap(),
            &crystal,
        )?;
    }
    //Successfully generate cosmic
    let cosmic_id =
        COSMIC_LENGTH.update::<_, StdError>(deps.storage, |id| Ok(id.add(Uint64::new(1))))?;
    let msg = generate_cosmic_mint_msg(String::from(cosmic_id), owner.to_string())?;
    let mint_msg = CosmosMsg::Wasm(Execute {
        contract_addr: state.cosmic_contract,
        msg: to_binary(&msg)?,
        funds: vec![],
    });
    Ok(Response::new().add_submessages(vec![SubMsg::new(fee_payout_msg), SubMsg::new(mint_msg)]))
}

fn execute_update_owner(
    deps: DepsMut,
    info: MessageInfo,
    new_owner: String,
) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    if info.sender.to_string() != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    state.owner = new_owner;
    STATE.save(deps.storage, &state)?;
    Ok(Response::default().add_attribute("new_owner", state.owner))
}

fn execute_update_state(
    deps: DepsMut,
    info: MessageInfo,
    cosmic_contract: String,
    drgn_recipient: String,
    allowed_cw20: Addr,
    attune_price: Uint128,
) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    if info.sender.to_string() != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    state.cosmic_contract = cosmic_contract;
    state.drgn_recipient = drgn_recipient;
    state.allowed_cw20 = allowed_cw20;
    state.attune_price = attune_price;
    STATE.save(deps.storage, &state)?;
    Ok(Response::default().add_attribute("new_cosmic_contract", state.cosmic_contract))
}
fn execute_burn(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
) -> Result<Response, ContractError> {
    let msg = Cw721ExecuteMsg::Burn { token_id };
    Cw721Contract::default()
        .execute(deps, env, info, msg)
        .unwrap();
    Ok(Response::new())
}

fn execute_mint(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    mut msg: CustomMintMsg,
) -> Result<Response, ContractError> {
    let mut kind = String::new();
    for item in &msg.extension {
        // iterate immutably
        let trait_type: String = item.clone().trait_type;
        let value: String = item.clone().value;

        match &trait_type[..] {
            "kind" => kind = value,
            _ => return Err(ContractError::UnexpectedTraitType { trait_type }),
        }
    }
    let id =
        CRYSTAL_INFO_SEQ.update::<_, StdError>(deps.storage, |id| Ok(id.add(Uint64::new(1))))?;
    let crystal = Crystal {
        token_id: id.to_string(),
        owner: msg.clone().base.owner,
        kind,
    };
    CRYSTAL_INFO.save(deps.storage, id.u64(), &crystal)?;
    msg.base.token_id = id.to_string();
    let mint_msg = Cw721ExecuteMsg::Mint(msg.base.clone());
    Cw721Contract::default()
        .execute(deps, env, info, mint_msg)
        .unwrap();
    Ok(Response::default()
        .add_attribute("new owner", crystal.owner.clone())
        .add_attribute("crystal id", crystal.token_id)
        .add_attribute("crystal kind", crystal.kind))
}

fn execute_generate_cosmic(
    mut deps: DepsMut,
    info: MessageInfo,
    env: Env,
    fire_id: Uint64,
    ice_id: Uint64,
    storm_id: Uint64,
    divine_id: Uint64,
    udin_id: Uint64,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let owner = info.clone().sender.to_string();

    if state.owner != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }
    //Check for different types exist
    let (fire, ice, storm, divine, udin) = (
        CRYSTAL_INFO.load(deps.storage, fire_id.u64())?,
        CRYSTAL_INFO.load(deps.storage, ice_id.u64())?,
        CRYSTAL_INFO.load(deps.storage, storm_id.u64())?,
        CRYSTAL_INFO.load(deps.storage, divine_id.u64())?,
        CRYSTAL_INFO.load(deps.storage, udin_id.u64())?,
    );
    let crystals = [fire, ice, storm, divine, udin];
    let kinds = ["fire", "ice", "storm", "divine", "udin"];
    for index in 0..5 {
        if crystals[index].owner != owner {
            return Err(ContractError::Unauthorized {});
        } else if crystals[index].kind != kinds[index] {
            return Err(ContractError::KindNotFound {});
        }
    }
    for index in 0..5 {
        let msg = Cw721ExecuteMsg::Burn {
            token_id: crystals[index].clone().token_id,
        };
        let res = Cw721Contract::default().execute(deps.branch(), env.clone(), info.clone(), msg);
        if res.is_err() {
            return Err(ContractError::NftContractError {
                method: "burn in generate cosmic".to_string(),
            });
        }
    }
    //Delete from the local list
    for index in 0..5 {
        let mut crystal = CRYSTAL_INFO.load(
            deps.storage,
            crystals[index].clone().token_id.parse::<u64>().unwrap(),
        )?;
        crystal.owner = "".to_string();
        CRYSTAL_INFO.save(
            deps.storage,
            crystals[index].clone().token_id.parse::<u64>().unwrap(),
            &crystal,
        )?;
    }
    //Successfully generate cosmic
    let cosmic_id =
        COSMIC_LENGTH.update::<_, StdError>(deps.storage, |id| Ok(id.add(Uint64::new(1))))?;
    let msg = generate_cosmic_mint_msg(String::from(cosmic_id), info.sender.to_string())?;
    Ok(Response::default().add_message(CosmosMsg::Wasm(Execute {
        contract_addr: state.cosmic_contract,
        msg: to_binary(&msg)?,
        funds: vec![],
    })))
}

fn execute_transfer_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: String,
    token_id: Uint64,
) -> Result<Response, ContractError> {
    let msg = Cw721ExecuteMsg::TransferNft {
        recipient: recipient.clone(),
        token_id: token_id.to_string(),
    };
    let mut crystal = CRYSTAL_INFO.load(deps.storage, token_id.u64())?;
    let valid_recipient = deps.api.addr_validate(&*recipient)?;
    crystal.owner = valid_recipient.to_string();
    CRYSTAL_INFO.save(deps.storage, token_id.u64(), &crystal)?;
    Cw721Contract::default()
        .execute(deps, env, info.clone(), msg)
        .unwrap();
    Ok(Response::default()
        .add_attribute("old owner", info.sender.to_string())
        .add_attribute("new owner", crystal.owner))
}

fn execute_revoke_all(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    operator: String,
) -> Result<Response, ContractError> {
    let msg = Cw721ExecuteMsg::RevokeAll { operator };
    let res = Cw721Contract::default().execute(deps, env, info, msg);
    if res.is_err() {
        return Err(ContractError::NftContractError {
            method: "revoke all".to_string(),
        });
    }
    Ok(Response::new())
}

fn execute_revoke(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    spender: String,
    token_id: String,
) -> Result<Response, ContractError> {
    let msg = Cw721ExecuteMsg::Revoke { spender, token_id };
    let res = Cw721Contract::default().execute(deps, env, info, msg);
    if res.is_err() {
        return Err(ContractError::NftContractError {
            method: "revoke".to_string(),
        });
    }
    Ok(Response::new())
}

fn execute_approve(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    spender: String,
    token_id: String,
    expires: Option<Expiration>,
) -> Result<Response, ContractError> {
    let msg = Cw721ExecuteMsg::Approve {
        spender,
        token_id,
        expires,
    };
    let res = Cw721Contract::default().execute(deps, env, info, msg);
    if res.is_err() {
        return Err(ContractError::NftContractError {
            method: "approve".to_string(),
        });
    }
    Ok(Response::new())
}

fn execute_approve_all(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    operator: String,
    expires: Option<Expiration>,
) -> Result<Response, ContractError> {
    let msg = Cw721ExecuteMsg::ApproveAll { operator, expires };
    let res = Cw721Contract::default().execute(deps, env, info, msg);
    if res.is_err() {
        return Err(ContractError::NftContractError {
            method: "approve all".to_string(),
        });
    }
    Ok(Response::new())
}

fn execute_send_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    contract: String,
    token_id: String,
    msg: Binary,
) -> Result<Response, ContractError> {
    let send_msg = Cw721ExecuteMsg::SendNft {
        contract,
        token_id,
        msg,
    };
    let res = Cw721Contract::default().execute(deps, env, info, send_msg);
    if res.is_err() {
        return Err(ContractError::NftContractError {
            method: "send nft".to_string(),
        });
    }
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::CollectionInfo {} => to_binary(&query_config(deps)?),
        QueryMsg::CrystalInfo { id } => to_binary(&query_crystal(deps, id)?),
        QueryMsg::RangeCrystals { start_after, limit } => {
            to_binary(&range_crystals(deps, start_after, limit)?)
        }
        QueryMsg::RangeUserCrystals {
            start_after,
            limit,
            owner,
        } => to_binary(&range_user_crystals(deps, start_after, limit, owner)?),
        QueryMsg::GetState {} => to_binary(&query_state(deps)?),
        _ => Cw721Contract::default().query(deps, env, msg.into()),
    }
}

fn query_crystal(deps: Deps, id: Uint64) -> StdResult<CrystalResponse> {
    let crystal = CRYSTAL_INFO.load(deps.storage, id.u64())?;
    Ok(CrystalResponse {
        token_id: crystal.token_id,
        owner: crystal.owner,
        kind: crystal.kind,
    })
}

fn query_config(deps: Deps) -> StdResult<CollectionInfoResponse> {
    let info = COLLECTION_INFO.load(deps.storage)?;

    Ok(CollectionInfoResponse {
        name: info.name,
        symbol: info.symbol,
        minter: info.minter,
        size: info.size,
    })
}

fn query_state(deps: Deps) -> StdResult<StateResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(StateResponse {
        owner: state.owner,
        cosmic_contract: state.cosmic_contract,
        drgn_recipient: state.drgn_recipient,
        allowed_cw20: state.allowed_cw20,
        attune_price: state.attune_price,
    })
}

// settings for pagination
const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

fn range_crystals(
    deps: Deps,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<CrystalListResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);
    let crystals: StdResult<Vec<_>> = CRYSTAL_INFO
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .collect();
    let res = CrystalListResponse {
        crystals: crystals?.into_iter().map(|l| l.1.into()).collect(),
    };
    Ok(res)
}

fn range_user_crystals(
    deps: Deps,
    start_after: Option<u64>,
    limit: Option<u32>,
    owner: String,
) -> StdResult<CrystalListResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);
    let crystals: StdResult<Vec<_>> = CRYSTAL_INFO
        .range(deps.storage, start, None, Order::Ascending)
        .filter(|r| r.as_ref().unwrap().1.owner == owner)
        .take(limit)
        .collect();
    let res = CrystalListResponse {
        crystals: crystals?.into_iter().map(|l| l.1.into()).collect(),
    };
    Ok(res)
}
