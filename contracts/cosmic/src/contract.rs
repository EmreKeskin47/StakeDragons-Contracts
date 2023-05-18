use crate::error::ContractError;
use crate::msg::{
    CollectionInfoResponse, CustomMintMsg, ExecuteMsg, Extension, InstantiateMsg, QueryMsg, ClaimMessage, Claim, StateResponse
};
use crate::state::{
    CollectionInfo, State, Cosmic, CosmicListResponse, CosmicResponse, COLLECTION_INFO, COSMIC_INFO,
    COSMIC_INFO_SEQ, STATE, MIN_STAKE_TIME,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::WasmMsg::Execute;
use cosmwasm_std::{to_binary, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo, Order, Response, StdError, StdResult, Uint64, Uint128, SubMsg};
use cw2::set_contract_version;
use std::ops::Add;

pub type Cw721Contract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty>;
pub type Cw721ExecuteMsg = cw721_base::ExecuteMsg<Extension>;
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
        reward_contract_address: msg.reward_contract_address,
        daily_income: msg.daily_income,
    };

    let collection_info = CollectionInfo {
        name: msg.base.name,
        symbol: msg.base.symbol,
        minter: minter.to_string(),
        description: "stake_dragons".to_string(),
        size: msg.size,
        base_price: msg.base_price,
    };

    STATE.save(deps.storage, &state)?;
    COLLECTION_INFO.save(deps.storage, &collection_info)?;
    COSMIC_INFO_SEQ.save(deps.storage, &Uint64::zero())?;
    MIN_STAKE_TIME.save(deps.storage, &Uint64::new(1209600))?;
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
        ExecuteMsg::UpdateOwner {new_owner} => execute_update_owner(deps, info, new_owner),
        ExecuteMsg::UpdateDailyIncome {new_daily_income} => execute_update_daily_income(deps, info, new_daily_income),
        ExecuteMsg::UpdateRewardContractAddress {new_address} => execute_update_reward_contract_address(deps, info, new_address),
        ExecuteMsg::UpdateMinStakeTime {time} => execute_update_min_stake_time(deps, info, time),
        ExecuteMsg::Mint(msg) => execute_mint(deps, env, info, msg),
        ExecuteMsg::StakeCosmic { token_id } => execute_stake_cosmic(deps, info, env, token_id),
        ExecuteMsg::StartUnstakingProcess { token_id } => {
            execute_start_unstake_process(deps, info, env, token_id)
        }
        ExecuteMsg::UnstakeCosmic { token_id } => execute_unstake_cosmic(deps, info, env, token_id),
        ExecuteMsg::ClaimReward { token_id } => execute_claim_reward(deps, info, env, token_id),
        ExecuteMsg::Claim { token_id } => execute_claim(deps, info, env, token_id),
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
    }
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

fn execute_update_daily_income(
    deps: DepsMut,
    info: MessageInfo,
    new_daily_income: Uint64,
) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    if info.sender.to_string() != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    state.daily_income = new_daily_income;
    STATE.save(deps.storage, &state)?;
    Ok(Response::default().add_attribute("new_owner", state.owner))
}

fn execute_update_reward_contract_address(
    deps: DepsMut,
    info: MessageInfo,
    new_address: String,
) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    if info.sender.to_string() != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    state.reward_contract_address = new_address;
    STATE.save(deps.storage, &state)?;
    Ok(Response::default().add_attribute("new_reward_contract_address", state.reward_contract_address))
}

fn execute_update_min_stake_time(
    deps: DepsMut,
    info: MessageInfo,
    time: Uint64,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender.to_string() != state.owner {
        return Err(ContractError::Unauthorized {})
    }
    MIN_STAKE_TIME.update::<_, StdError>(deps.storage, |_min_stake_time| Ok(time))?;

    Ok(Response::default().add_attribute("min_stake_time", time))
}

//Add burning for your local list here
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
    let state = STATE.load(deps.storage)?;
    let daily_income: String = state.daily_income.to_string();
    let id =
        COSMIC_INFO_SEQ.update::<_, StdError>(deps.storage, |id| Ok(id.add(Uint64::new(1))))?;
    let cosmic = Cosmic {
        token_id: id.to_string(),
        owner: msg.clone().base.owner,
        daily_income,
        is_staked: false,
        stake_start_time: Uint64::zero(),
        reward_start_time: Uint64::zero(),
        unstaking_start_time: Uint64::zero(),
        unstaking_process: false,
        reward_end_time: Uint64::zero(),
    };
    COSMIC_INFO.save(deps.storage, id.u64(), &cosmic)?;
    msg.base.token_id = id.to_string();
    let mint_msg = Cw721ExecuteMsg::Mint(msg.base.clone());
    Cw721Contract::default()
        .execute(deps, env, info, mint_msg)
        .unwrap();
    Ok(Response::default()
        .add_attribute("new owner", cosmic.owner.clone())
        .add_attribute("cosmic id", cosmic.token_id)
        .add_attribute("cosmic daily income", cosmic.daily_income)
        .add_attribute("stake_start_time", cosmic.stake_start_time.to_string())
        .add_attribute("reward_start_time", cosmic.reward_start_time.to_string())
        .add_attribute("is_staked, {}", cosmic.is_staked.to_string())
        .add_attribute("unstaking_start_time, {}", cosmic.unstaking_start_time.to_string())
        .add_attribute("unstaking_process, {}", cosmic.unstaking_start_time.to_string())
        .add_attribute("reward_end_time, {}", cosmic.unstaking_start_time.to_string()))
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
    let mut cosmic = COSMIC_INFO.load(deps.storage, token_id.u64())?;
    let is_staked = cosmic.clone().is_staked;
    if is_staked {
        return Err(ContractError::StakedCosmicCantBeTransferred {});
    }
    let valid_recipient = deps.api.addr_validate(&*recipient)?;
    cosmic.owner = valid_recipient.to_string();
    COSMIC_INFO.save(deps.storage, token_id.u64(), &cosmic)?;
    Cw721Contract::default()
        .execute(deps, env, info.clone(), msg)
        .unwrap();

    Ok(Response::default()
        .add_attribute("old owner", info.sender.to_string())
        .add_attribute("new owner", cosmic.owner))
}

pub fn execute_stake_cosmic(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    token_id: Uint64,
) -> Result<Response, ContractError> {
    let mut cosmic = COSMIC_INFO.load(deps.storage, token_id.u64())?;
    if cosmic.is_staked {
        return Err(ContractError::CosmicAlreadyStaked {});
    }
    let _is_owner = cosmic.clone().is_owner(info.sender.to_string())?;
    cosmic.is_staked = true;
    let now = Uint64::new(env.block.time.seconds());
    cosmic.stake_start_time = now;
    cosmic.reward_start_time = now;
    COSMIC_INFO.save(deps.storage, token_id.u64(), &cosmic)?;
    Ok(Response::default()
        .add_attribute("token_id", cosmic.token_id.to_string())
        .add_attribute("is_staked", cosmic.is_staked.to_string())
        .add_attribute("start_time", cosmic.stake_start_time)
        .add_attribute("reward_start_time", cosmic.reward_start_time))
}

fn execute_start_unstake_process(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    token_id: Uint64,
) -> Result<Response, ContractError> {
    let mut cosmic = COSMIC_INFO.load(deps.storage, token_id.u64())?;
    if cosmic.unstaking_process {
        return Err(ContractError::OngoingUnstakingProcess {});
    }
    if !cosmic.is_staked {
        return Err(ContractError::CosmicNotStaked {});
    }
    if cosmic.owner != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }
    cosmic.unstaking_process = true;
    cosmic.unstaking_start_time = Uint64::new(env.block.time.seconds());
    cosmic.reward_end_time = Uint64::new(env.block.time.seconds());
    COSMIC_INFO.save(deps.storage, token_id.u64(), &cosmic)?;
    Ok(Response::default()
        .add_attribute("token_id", cosmic.clone().token_id.to_string())
        .add_attribute("unstaking_start_time", cosmic.unstaking_start_time)
        .add_attribute("reward_end_time", cosmic.reward_end_time))
}

fn execute_unstake_cosmic(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    token_id: Uint64,
) -> Result<Response, ContractError> {
    let mut cosmic = COSMIC_INFO.load(deps.storage, token_id.u64())?;
    if !cosmic.is_staked {
        return Err(ContractError::CosmicNotStaked {});
    }
    if cosmic.owner != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }
    if !cosmic.unstaking_process {
        return Err(ContractError::UnstakingProcessIsNotStarted {});
    }
    let now = Uint64::new(env.block.time.seconds());
    let min_stake_time = MIN_STAKE_TIME.load(deps.storage)?;
    if now.checked_sub(cosmic.unstaking_start_time)? < min_stake_time
        || cosmic.unstaking_start_time.is_zero()
    {
        return Err(ContractError::MinUnstakingTimeRequired {});
    }
    cosmic.is_staked = false;
    cosmic.stake_start_time = Uint64::zero();
    cosmic.reward_start_time = Uint64::zero();
    cosmic.unstaking_process = false;
    cosmic.unstaking_start_time = Uint64::zero();
    COSMIC_INFO.save(deps.storage, token_id.u64(), &cosmic)?;
    Ok(Response::default()
        .add_attribute("token_id", cosmic.clone().token_id.to_string())
        .add_attribute("is_staked", cosmic.clone().is_staked.to_string())
        .add_attribute(
            "stake_start_time",
            cosmic.clone().stake_start_time.to_string(),
        )
        .add_attribute(
            "reward_start_time",
            cosmic.clone().reward_start_time.to_string(),
        )
        .add_attribute(
            "unstaking_process",
            cosmic.clone().unstaking_process.to_string(),
        )
        .add_attribute(
            "unstaking_start_time",
            cosmic.clone().unstaking_start_time.to_string(),
        ))
}

fn execute_claim_reward(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    token_id: Uint64,
) -> Result<Response, ContractError> {
    let mut cosmic = COSMIC_INFO.load(deps.storage, token_id.u64())?;
    if !cosmic.is_staked {
        return Err(ContractError::CosmicNotStaked {});
    }
    if cosmic.owner != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }
    //Reward calculation
    let now = env.block.time.seconds();
    let reward: Uint128;
    if cosmic.unstaking_process {
            let second_difference = Uint128::new(cosmic.reward_end_time.u64() as u128).checked_sub(Uint128::new(cosmic.reward_start_time.u64() as u128))?;
            let second_difference_multiplied = second_difference.checked_mul(Uint128::new(1000000))?;
            let second_difference_multiplied_daily_income = second_difference_multiplied.checked_mul(Uint128::new(cosmic.daily_income.parse::<u128>().unwrap()))?;
            reward = second_difference_multiplied_daily_income.checked_div(Uint128::new(86400)).unwrap_or(Uint128::zero());
    } else {
            let second_difference = Uint128::new(now as u128).checked_sub(Uint128::new(cosmic.reward_start_time.u64() as u128))?;
            let second_difference_multiplied = second_difference.checked_mul(Uint128::new(1000000))?;
            let second_difference_multiplied_daily_income = second_difference_multiplied.checked_mul(Uint128::new(cosmic.daily_income.parse::<u128>().unwrap()))?;
            reward = second_difference_multiplied_daily_income.checked_div(Uint128::new(86400)).unwrap_or(Uint128::zero());
    }
    let state = STATE.load(deps.storage)?;
    let msg = ClaimMessage {
        claim: Claim {
            recipient: info.sender.to_string(),
            amount: reward,
        }
    };

    let claim_reward_msg = CosmosMsg::Wasm(Execute {
        contract_addr: state.reward_contract_address,
        msg: to_binary(&msg)?,
        funds: vec![],
    });
    if cosmic.unstaking_process {
        cosmic.reward_start_time = Uint64::zero();
        cosmic.reward_end_time = Uint64::zero();
    } else {
        cosmic.reward_start_time = Uint64::new(env.block.time.seconds());
    }
    COSMIC_INFO.save(deps.storage, token_id.u64(), &cosmic)?;
    Ok(Response::new().add_submessages(vec![
        SubMsg::new(claim_reward_msg),
    ]))
}

fn execute_claim(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    token_id: Uint64,
) -> Result<Response, ContractError> {
    let mut cosmic = COSMIC_INFO.load(deps.storage, token_id.u64())?;
    if !cosmic.is_staked {
        return Err(ContractError::CosmicNotStaked {});
    }
    if cosmic.owner != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }
    if cosmic.unstaking_process {
        cosmic.reward_start_time = Uint64::zero();
        cosmic.reward_end_time = Uint64::zero();
    } else {
        cosmic.reward_start_time = Uint64::new(env.block.time.seconds());
    }
    COSMIC_INFO.save(deps.storage, token_id.u64(), &cosmic)?;
    Ok(Response::new().add_attribute("reward_start_time", cosmic.reward_start_time)
        .add_attribute("reward_end_time", cosmic.reward_end_time))
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
        QueryMsg::CosmicInfo { id } => to_binary(&query_cosmic(deps, id)?),
        QueryMsg::RangeCosmics { start_after, limit } => {
            to_binary(&range_cosmics(deps, start_after, limit)?)
        }
        QueryMsg::RangeUserCosmics {
            start_after,
            limit,
            owner,
        } => to_binary(&range_user_cosmics(deps, start_after, limit, owner)?),
        QueryMsg::CalculateReward { token_id } => {
            to_binary(&query_calculate_reward(deps, env, token_id)?)
        }
        QueryMsg::State {} => to_binary(&query_state(deps)?),
        _ => Cw721Contract::default().query(deps, env, msg.into()),
    }
}

fn query_calculate_reward(deps: Deps, env: Env, token_id: Uint64) -> StdResult<Uint64> {
    let cosmic = COSMIC_INFO.load(deps.storage, token_id.u64()).unwrap();
    let now = Uint64::new(env.block.time.seconds());
    let reward: Uint64;
    if cosmic.unstaking_process {
            let second_difference = cosmic.reward_end_time.checked_sub(cosmic.reward_start_time)?;
            let second_difference_multiplied = second_difference.checked_mul(Uint64::new(1000000))?;
            let second_difference_multiplied_daily_income = second_difference_multiplied.checked_mul(Uint64::new(cosmic.daily_income.parse::<u64>().unwrap()))?;
            reward = second_difference_multiplied_daily_income.checked_div(Uint64::new(86400))?;
    } else {
            let second_difference = now.checked_sub(cosmic.reward_start_time)?;
            let second_difference_multiplied = second_difference.checked_mul(Uint64::new(1000000))?;
            let second_difference_multiplied_daily_income = second_difference_multiplied.checked_mul(Uint64::new(cosmic.daily_income.parse::<u64>().unwrap()))?;
            reward = second_difference_multiplied_daily_income.checked_div(Uint64::new(86400))?;
    }
    Ok(reward)
}

fn query_cosmic(deps: Deps, id: Uint64) -> StdResult<CosmicResponse> {
    let cosmic = COSMIC_INFO.load(deps.storage, id.u64())?;
    Ok(CosmicResponse {
        token_id: cosmic.token_id,
        owner: cosmic.owner,
        daily_income: cosmic.daily_income,
        is_staked: cosmic.is_staked,
        stake_start_time: cosmic.stake_start_time,
        reward_start_time: cosmic.reward_start_time,
        unstaking_start_time: cosmic.unstaking_start_time,
        unstaking_process: cosmic.unstaking_process,
        reward_end_time: cosmic.reward_end_time,
    })
}

fn query_config(deps: Deps) -> StdResult<CollectionInfoResponse> {
    let info = COLLECTION_INFO.load(deps.storage)?;

    Ok(CollectionInfoResponse {
        name: info.name,
        symbol: info.symbol,
        minter: info.minter,
        description: info.description,
        size: info.size,
        base_price: info.base_price,
    })
}

fn query_state(deps: Deps) -> StdResult<StateResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(StateResponse {
        owner: state.owner,
        reward_contract_address: state.reward_contract_address
    })
}

// settings for pagination
const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

fn range_cosmics(
    deps: Deps,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<CosmicListResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);
    let cosmics: StdResult<Vec<_>> = COSMIC_INFO
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .collect();
    let res = CosmicListResponse {
        cosmics: cosmics?.into_iter().map(|l| l.1.into()).collect(),
    };
    Ok(res)
}

fn range_user_cosmics(
    deps: Deps,
    start_after: Option<u64>,
    limit: Option<u32>,
    owner: String,
) -> StdResult<CosmicListResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);
    let cosmics: StdResult<Vec<_>> = COSMIC_INFO
        .range(deps.storage, start, None, Order::Ascending)
        .filter(|r| r.as_ref().unwrap().1.owner == owner)
        .take(limit)
        .collect();
    let res = CosmicListResponse {
        cosmics: cosmics?.into_iter().map(|l| l.1.into()).collect(),
    };
    Ok(res)
}
