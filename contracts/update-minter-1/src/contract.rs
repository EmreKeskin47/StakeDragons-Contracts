#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::WasmMsg::Execute;
use cosmwasm_std::{
    from_slice, to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, QueryRequest,
    Response, StdError, StdResult, SubMsg, Uint128, Uint64, WasmQuery,
};
use std::ops::Add;

use crate::msg::{
    DragonInfoMsg, DragonResponse, ExecuteMsg, GetDragonInfoMsg, GetStateResponse, InstantiateMsg,
    MinterDragonBirth, MinterEditContracts, MinterEditState, QueryMsg, ReceiveMsg,
};
use crate::state::{
    Contracts, RequiredMinMax, State, UpdatedStats, CONTRACTS, REQUIRED_MIN_MAX, STATE,
    UPDATED_DRAGON_COUNT, UPDATED_STATS,
};

use crate::error::ContractError;
use crate::helper::{generate_updated_dragon_mint_msg, Metadata};

use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};
pub type Cw721ExecuteMsg = cw721_base::ExecuteMsg<Metadata>;

use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:update-minter-1";
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
        drgn_contract: msg.drgn_contract,
        allowed_cw20: msg.allowed_cw20,
        allowed_operators: msg.allowed_operators,
        random_key: msg.random_key,
        drgn_rac: msg.drgn_rac,
        season: msg.season,
    };

    let contracts = Contracts {
        dragon: msg.dragon,
        updated_dragon: msg.updated_dragon,
        egg_minter: msg.egg_minter,
        drgn_recipient: msg.drgn_recipient,
        cw20_recipient: msg.cw20_recipient,
    };

    let stats = UpdatedStats {
        common_reward: Uint64::new(2),
        common_ovulation: Uint64::new(20),
        uncommon_reward: Uint64::new(4),
        uncommon_ovulation: Uint64::new(15),
        rare_reward: Uint64::new(8),
        rare_ovulation: Uint64::new(10),
        epic_reward: Uint64::new(20),
        epic_ovulation: Uint64::new(7),
        legendary_reward: Uint64::new(15),
        legendary_ovulation: Uint64::new(5),
    };

    let values = RequiredMinMax {
        common_min: Uint128::new(45_000_000),
        common_max: Uint128::new(90_000_000),
        uncommon_min: Uint128::new(90_000_000),
        uncommon_max: Uint128::new(180_000_000),
        rare_min: Uint128::new(180_000_000),
        rare_max: Uint128::new(360_000_000),
        epic_min: Uint128::new(450_000_000),
        epic_max: Uint128::new(900_000_000),
        legendary_min: Uint128::new(900_000_000),
        legendary_max: Uint128::new(1_800_000_000),
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;
    CONTRACTS.save(deps.storage, &contracts)?;
    UPDATED_DRAGON_COUNT.save(deps.storage, &Uint64::new(0))?;
    UPDATED_STATS.save(deps.storage, &stats)?;
    REQUIRED_MIN_MAX.save(deps.storage, &values)?;

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
        ExecuteMsg::EditState {
            owner,
            drgn_contract,
            allowed_cw20,
            allowed_operators,
            random_key,
            drgn_rac,
            season,
        } => execute_edit_state(
            deps,
            info,
            owner,
            drgn_contract,
            allowed_cw20,
            allowed_operators,
            random_key,
            drgn_rac,
            season,
        ),
        ExecuteMsg::EditContracts {
            dragon,
            updated_dragon,
            egg_minter,
            drgn_recipient,
            cw20_recipient,
        } => execute_edit_contracts(
            deps,
            info,
            dragon,
            updated_dragon,
            egg_minter,
            drgn_recipient,
            cw20_recipient,
        ),
        ExecuteMsg::EditStats {
            common_reward,
            common_ovulation,
            uncommon_reward,
            uncommon_ovulation,
            rare_reward,
            rare_ovulation,
            epic_reward,
            epic_ovulation,
            legendary_reward,
            legendary_ovulation,
        } => execute_edit_stats(
            deps,
            info,
            common_reward,
            common_ovulation,
            uncommon_reward,
            uncommon_ovulation,
            rare_reward,
            rare_ovulation,
            epic_reward,
            epic_ovulation,
            legendary_reward,
            legendary_ovulation,
        ),
        ExecuteMsg::UpgradeAdmin {
            owner,
            id_1,
            id_2,
            id_3,
            rarity,
        } => execute_update_nft_admin(deps, env, info, owner, id_1, id_2, id_3, &rarity),
        ExecuteMsg::Receive(cw20_receive_msg) => execute_receive(deps, env, info, cw20_receive_msg),
        ExecuteMsg::EditOldMinterState(msg) => execute_edit_old_minter_state(deps, info, msg),
        ExecuteMsg::EditOldMinterContracts(msg) => {
            execute_edit_old_minter_contracts(deps, info, msg)
        }
        ExecuteMsg::OldMinterDragonBirth(msg) => execute_old_minter_dragon_birth(deps, info, msg),
        ExecuteMsg::EditMinMax {
            common_min,
            common_max,
            uncommon_min,
            uncommon_max,
            rare_min,
            rare_max,
            epic_min,
            epic_max,
            legendary_min,
            legendary_max,
        } => execute_edit_min_max(
            deps,
            info,
            common_min,
            common_max,
            uncommon_min,
            uncommon_max,
            rare_min,
            rare_max,
            epic_min,
            epic_max,
            legendary_min,
            legendary_max,
        ),
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
    match msg {
        ReceiveMsg::Upgrade {
            owner,
            id_1,
            id_2,
            id_3,
            rarity,
            res,
        } => execute_update_nft_cw20(
            deps, _env, info, amount, owner, id_1, id_2, id_3, &rarity, res,
        ),
    }
}

pub fn execute_update_nft_cw20(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
    owner: String,
    id_1: Uint64,
    id_2: Uint64,
    id_3: Uint64,
    rarity: &str,
    res: String,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let contracts = CONTRACTS.load(deps.storage)?;
    let stats = UPDATED_STATS.load(deps.storage)?;
    let min_max = REQUIRED_MIN_MAX.load(deps.storage)?;

    //check incoming cw20
    if info.sender != state.allowed_cw20 && info.sender != state.drgn_contract {
        return Err(ContractError::CW20TokenNotAllowed {
            sent: info.sender.to_string(),
            need: state.drgn_contract.to_string(),
        });
    }

    let mut min_amount: Uint128;
    let mut max_amount: Uint128;
    let ovulation: String;
    let daily_reward: String;

    //check rarity
    match rarity {
        "common" => {
            min_amount = min_max.common_min;
            max_amount = min_max.common_max;
            ovulation = stats.common_ovulation.to_string();
            daily_reward = stats.common_reward.to_string()
        }
        "uncommon" => {
            min_amount = min_max.uncommon_min;
            max_amount = min_max.uncommon_max;
            ovulation = stats.uncommon_ovulation.to_string();
            daily_reward = stats.uncommon_reward.to_string()
        }
        "rare" => {
            min_amount = min_max.rare_min;
            max_amount = min_max.rare_max;
            ovulation = stats.rare_ovulation.to_string();
            daily_reward = stats.rare_reward.to_string()
        }
        "epic" => {
            min_amount = min_max.epic_min;
            max_amount = min_max.epic_max;
            ovulation = stats.epic_ovulation.to_string();
            daily_reward = stats.epic_reward.to_string()
        }
        "legendary" => {
            min_amount = min_max.legendary_min;
            max_amount = min_max.legendary_max;
            ovulation = stats.legendary_ovulation.to_string();
            daily_reward = stats.legendary_reward.to_string()
        }
        _ => return Err(ContractError::RarityNotSupported {}),
    }

    if info.sender == state.allowed_cw20 {
        min_amount = min_amount.checked_mul(state.drgn_rac)?;
        min_amount = min_amount.checked_div(Uint128::new(100))?;
        max_amount = max_amount.checked_mul(state.drgn_rac)?;
        max_amount = max_amount.checked_div(Uint128::new(100))?;
    }

    //Check for amount
    if amount.clone() > max_amount || amount.clone() < min_amount {
        return Err(ContractError::InvalidAmountSent {
            max: max_amount,
            min: min_amount,
            sent: amount.clone(),
        });
    }

    //Check if rarity is correct and dragons are not staked
    let msg_1 = GetDragonInfoMsg {
        DragonInfo: DragonInfoMsg { id: id_1.clone() },
    };
    let msg_2 = GetDragonInfoMsg {
        DragonInfo: DragonInfoMsg { id: id_2.clone() },
    };
    let msg_3 = GetDragonInfoMsg {
        DragonInfo: DragonInfoMsg { id: id_3.clone() },
    };
    let nft_info_1: DragonResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: String::from(contracts.dragon.clone()),
        msg: to_binary(&msg_1)?,
    }))?;
    let nft_info_2: DragonResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: String::from(contracts.dragon.clone()),
        msg: to_binary(&msg_2)?,
    }))?;
    let nft_info_3: DragonResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: String::from(contracts.dragon.clone()),
        msg: to_binary(&msg_3)?,
    }))?;

    if nft_info_1.is_staked || nft_info_2.is_staked || nft_info_3.is_staked {
        return Err(ContractError::DragonIsStaked {});
    }

    if nft_info_1.kind != rarity || nft_info_2.kind != rarity || nft_info_3.kind != rarity {
        return Err(ContractError::DragonRarityMismatch {});
    }

    //Send cw20 to recipients
    let mut recipient = "".to_string();
    let mut cw20_target = state.clone().drgn_contract;
    if info.sender == state.drgn_contract {
        recipient = contracts.drgn_recipient;
    } else if info.sender == state.allowed_cw20 {
        recipient = contracts.cw20_recipient;
        cw20_target = state.allowed_cw20;
    }

    let cw20_execute_msg_fp = Cw20ExecuteMsg::Transfer { recipient, amount };
    let execute_payment_msg = CosmosMsg::Wasm(Execute {
        contract_addr: cw20_target.to_string(),
        msg: to_binary(&cw20_execute_msg_fp)?,
        funds: vec![],
    });

    //Burn all given dragons
    let burn_msg_1 = Cw721ExecuteMsg::Burn {
        token_id: id_1.to_string(),
    };
    let burn_msg_2 = Cw721ExecuteMsg::Burn {
        token_id: id_2.to_string(),
    };
    let burn_msg_3 = Cw721ExecuteMsg::Burn {
        token_id: id_3.to_string(),
    };

    let execute_burn_1 = CosmosMsg::Wasm(Execute {
        contract_addr: contracts.dragon.to_string(),
        msg: to_binary(&burn_msg_1)?,
        funds: vec![],
    });
    let execute_burn_2 = CosmosMsg::Wasm(Execute {
        contract_addr: contracts.dragon.to_string(),
        msg: to_binary(&burn_msg_2)?,
        funds: vec![],
    });
    let execute_burn_3 = CosmosMsg::Wasm(Execute {
        contract_addr: contracts.dragon.to_string(),
        msg: to_binary(&burn_msg_3)?,
        funds: vec![],
    });

    let mut submessages = vec![
        SubMsg::new(execute_burn_1),
        SubMsg::new(execute_burn_2),
        SubMsg::new(execute_burn_3),
        SubMsg::new(execute_payment_msg),
    ];

    // Pick random index from given array for success/fail
    let key1 = state.random_key / 10000;
    let key2 = state.random_key % 100;
    let secret = key1 + key2;
    let type_id = res.chars().nth(secret as usize).unwrap();

    let success: bool;
    match type_id {
        'j' => success = false,
        '2' => success = false,
        '8' => success = false,
        'H' => success = false,
        _ => success = true,
    };

    if success == true {
        //Mint msg
        UPDATED_DRAGON_COUNT
            .update::<_, StdError>(deps.storage, |id| Ok(id.add(Uint64::new(1))))?;
        let id = UPDATED_DRAGON_COUNT.load(deps.storage)?;

        let mint_msg = generate_updated_dragon_mint_msg(
            id,
            owner,
            nft_info_1.kind,
            ovulation,
            daily_reward,
            state.season,
        )?;
        let execute_mint_msg = CosmosMsg::Wasm(Execute {
            contract_addr: contracts.updated_dragon,
            msg: to_binary(&mint_msg)?,
            funds: vec![],
        });

        submessages.push(SubMsg::new(execute_mint_msg))
    }

    Ok(Response::new()
        .add_submessages(submessages)
        .add_attribute("success", success.to_string()))
}

pub fn execute_update_nft_admin(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    owner: String,
    id_1: Uint64,
    id_2: Uint64,
    id_3: Uint64,
    rarity: &str,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let stats = UPDATED_STATS.load(deps.storage)?;

    if state.allowed_operators.contains(&info.sender.to_string()) == false {
        return Err(ContractError::Unauthorized {
            msg: "Only allowed operators can execute this message".to_string(),
        });
    }

    let contracts = CONTRACTS.load(deps.storage)?;

    //Check if rarity is correct and dragons are not staked
    let msg_1 = GetDragonInfoMsg {
        DragonInfo: DragonInfoMsg { id: id_1.clone() },
    };
    let msg_2 = GetDragonInfoMsg {
        DragonInfo: DragonInfoMsg { id: id_2.clone() },
    };
    let msg_3 = GetDragonInfoMsg {
        DragonInfo: DragonInfoMsg { id: id_3.clone() },
    };
    let nft_info_1: DragonResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: String::from(contracts.dragon.clone()),
        msg: to_binary(&msg_1)?,
    }))?;
    let nft_info_2: DragonResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: String::from(contracts.dragon.clone()),
        msg: to_binary(&msg_2)?,
    }))?;
    let nft_info_3: DragonResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: String::from(contracts.dragon.clone()),
        msg: to_binary(&msg_3)?,
    }))?;

    if nft_info_1.is_staked || nft_info_2.is_staked || nft_info_3.is_staked {
        return Err(ContractError::DragonIsStaked {});
    }

    if nft_info_1.kind != rarity || nft_info_2.kind != rarity || nft_info_3.kind != rarity {
        return Err(ContractError::DragonRarityMismatch {});
    }

    //Burn all given dragons
    let burn_msg_1 = Cw721ExecuteMsg::Burn {
        token_id: id_1.to_string(),
    };
    let burn_msg_2 = Cw721ExecuteMsg::Burn {
        token_id: id_2.to_string(),
    };
    let burn_msg_3 = Cw721ExecuteMsg::Burn {
        token_id: id_3.to_string(),
    };

    let execute_burn_1 = CosmosMsg::Wasm(Execute {
        contract_addr: contracts.dragon.to_string(),
        msg: to_binary(&burn_msg_1)?,
        funds: vec![],
    });
    let execute_burn_2 = CosmosMsg::Wasm(Execute {
        contract_addr: contracts.dragon.to_string(),
        msg: to_binary(&burn_msg_2)?,
        funds: vec![],
    });
    let execute_burn_3 = CosmosMsg::Wasm(Execute {
        contract_addr: contracts.dragon.to_string(),
        msg: to_binary(&burn_msg_3)?,
        funds: vec![],
    });

    //Mint msg
    UPDATED_DRAGON_COUNT.update::<_, StdError>(deps.storage, |id| Ok(id.add(Uint64::new(1))))?;
    let id = UPDATED_DRAGON_COUNT.load(deps.storage)?;
    let ovulation: String;
    let daily_reward: String;

    //check rarity
    match rarity {
        "common" => {
            ovulation = stats.common_ovulation.to_string();
            daily_reward = stats.common_reward.to_string()
        }
        "uncommon" => {
            ovulation = stats.uncommon_ovulation.to_string();
            daily_reward = stats.uncommon_reward.to_string()
        }
        "rare" => {
            ovulation = stats.rare_ovulation.to_string();
            daily_reward = stats.rare_reward.to_string()
        }
        "epic" => {
            ovulation = stats.epic_ovulation.to_string();
            daily_reward = stats.epic_reward.to_string()
        }
        "legendary" => {
            ovulation = stats.legendary_ovulation.to_string();
            daily_reward = stats.legendary_reward.to_string()
        }
        _ => return Err(ContractError::RarityNotSupported {}),
    }

    let mint_msg = generate_updated_dragon_mint_msg(
        id,
        owner,
        nft_info_1.kind,
        ovulation,
        daily_reward,
        state.season,
    )?;
    let execute_mint_msg = CosmosMsg::Wasm(Execute {
        contract_addr: contracts.updated_dragon,
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    });

    let submessages = vec![
        SubMsg::new(execute_burn_1),
        SubMsg::new(execute_burn_2),
        SubMsg::new(execute_burn_3),
        SubMsg::new(execute_mint_msg),
    ];

    Ok(Response::new().add_submessages(submessages))
}
pub fn execute_edit_state(
    deps: DepsMut,
    info: MessageInfo,
    owner: String,
    drgn_contract: Addr,
    allowed_cw20: Addr,
    allowed_operators: Vec<String>,
    random_key: i32,
    drgn_rac: Uint128,
    season: String,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if state.allowed_operators.contains(&info.sender.to_string()) == false {
        return Err(ContractError::Unauthorized {
            msg: "state can only be edited by the owner".to_string(),
        });
    }
    let new = State {
        owner,
        drgn_contract,
        allowed_cw20,
        allowed_operators,
        random_key,
        drgn_rac,
        season,
    };
    STATE.save(deps.storage, &new)?;
    Ok(Response::new())
}

pub fn execute_edit_min_max(
    deps: DepsMut,
    info: MessageInfo,
    common_min: Uint128,
    common_max: Uint128,
    uncommon_min: Uint128,
    uncommon_max: Uint128,
    rare_min: Uint128,
    rare_max: Uint128,
    epic_min: Uint128,
    epic_max: Uint128,
    legendary_min: Uint128,
    legendary_max: Uint128,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if state.allowed_operators.contains(&info.sender.to_string()) == false {
        return Err(ContractError::Unauthorized {
            msg: "state can only be edited by the owner".to_string(),
        });
    }

    let stats = RequiredMinMax {
        common_min,
        common_max,
        uncommon_min,
        uncommon_max,
        rare_min,
        rare_max,
        epic_min,
        epic_max,
        legendary_min,
        legendary_max,
    };

    REQUIRED_MIN_MAX.save(deps.storage, &stats)?;
    Ok(Response::new())
}

pub fn execute_edit_stats(
    deps: DepsMut,
    info: MessageInfo,
    common_reward: Uint64,
    common_ovulation: Uint64,
    uncommon_reward: Uint64,
    uncommon_ovulation: Uint64,
    rare_reward: Uint64,
    rare_ovulation: Uint64,
    epic_reward: Uint64,
    epic_ovulation: Uint64,
    legendary_reward: Uint64,
    legendary_ovulation: Uint64,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if state.allowed_operators.contains(&info.sender.to_string()) == false {
        return Err(ContractError::Unauthorized {
            msg: "state can only be edited by the owner".to_string(),
        });
    }

    let stats = UpdatedStats {
        common_reward,
        common_ovulation,
        uncommon_reward,
        uncommon_ovulation,
        rare_reward,
        rare_ovulation,
        epic_reward,
        epic_ovulation,
        legendary_reward,
        legendary_ovulation,
    };

    UPDATED_STATS.save(deps.storage, &stats)?;
    Ok(Response::new())
}

pub fn execute_edit_contracts(
    deps: DepsMut,
    info: MessageInfo,
    dragon: Addr,
    updated_dragon: String,
    egg_minter: String,
    drgn_recipient: String,
    cw20_recipient: String,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if state.allowed_operators.contains(&info.sender.to_string()) == false {
        return Err(ContractError::Unauthorized {
            msg: "state can only be edited by the owner".to_string(),
        });
    }
    let new = Contracts {
        dragon,
        updated_dragon,
        egg_minter,
        drgn_recipient,
        cw20_recipient,
    };

    CONTRACTS.save(deps.storage, &new)?;
    Ok(Response::new())
}

pub fn execute_edit_old_minter_state(
    deps: DepsMut,
    info: MessageInfo,
    msg: MinterEditState,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let contracts = CONTRACTS.load(deps.storage)?;

    if state.allowed_operators.contains(&info.sender.to_string()) == false {
        return Err(ContractError::Unauthorized {
            msg: "state can only be edited by the owner".to_string(),
        });
    }

    let msg = CosmosMsg::Wasm(Execute {
        contract_addr: contracts.egg_minter,
        msg: to_binary(&msg)?,
        funds: vec![],
    });

    Ok(Response::new().add_message(msg))
}

pub fn execute_edit_old_minter_contracts(
    deps: DepsMut,
    info: MessageInfo,
    msg: MinterEditContracts,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let contracts = CONTRACTS.load(deps.storage)?;

    if state.allowed_operators.contains(&info.sender.to_string()) == false {
        return Err(ContractError::Unauthorized {
            msg: "state can only be edited by the owner".to_string(),
        });
    }

    let msg = CosmosMsg::Wasm(Execute {
        contract_addr: contracts.egg_minter,
        msg: to_binary(&msg)?,
        funds: vec![],
    });

    Ok(Response::new().add_message(msg))
}

pub fn execute_old_minter_dragon_birth(
    deps: DepsMut,
    info: MessageInfo,
    msg: MinterDragonBirth,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let contracts = CONTRACTS.load(deps.storage)?;

    if state.allowed_operators.contains(&info.sender.to_string()) == false {
        return Err(ContractError::Unauthorized {
            msg: "state can only be edited by the owner".to_string(),
        });
    }

    let msg = CosmosMsg::Wasm(Execute {
        contract_addr: contracts.egg_minter,
        msg: to_binary(&msg)?,
        funds: vec![],
    });

    Ok(Response::new().add_message(msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetState {} => to_binary(&query::state(deps)?),
        QueryMsg::GetStats {} => to_binary(&query::stats(deps)?),
        QueryMsg::GetMinMax {} => to_binary(&query::token_amount(deps)?),
    }
}

pub mod query {
    use super::*;
    use crate::msg::{GetMinMaxResponse, GetStatsResponse};

    pub fn state(deps: Deps) -> StdResult<GetStateResponse> {
        let state = STATE.load(deps.storage)?;
        let contract = CONTRACTS.load(deps.storage)?;
        Ok(GetStateResponse {
            owner: state.owner,
            drgn_contract: state.drgn_contract,
            allowed_cw20: state.allowed_cw20,
            allowed_operators: state.allowed_operators,
            random_key: state.random_key,
            drgn_rac: state.drgn_rac,
            season: state.season,
            dragon: contract.dragon,
            updated_dragon: contract.updated_dragon,
            egg_minter: contract.egg_minter,
            drgn_recipient: contract.drgn_recipient,
            cw20_recipient: contract.cw20_recipient,
        })
    }

    pub fn stats(deps: Deps) -> StdResult<GetStatsResponse> {
        let stats = UPDATED_STATS.load(deps.storage)?;
        Ok(GetStatsResponse {
            common_reward: stats.common_reward,
            common_ovulation: stats.common_ovulation,
            uncommon_reward: stats.uncommon_reward,
            uncommon_ovulation: stats.uncommon_ovulation,
            rare_reward: stats.rare_reward,
            rare_ovulation: stats.rare_ovulation,
            epic_reward: stats.epic_reward,
            epic_ovulation: stats.epic_ovulation,
            legendary_reward: stats.legendary_reward,
            legendary_ovulation: stats.legendary_ovulation,
        })
    }

    pub fn token_amount(deps: Deps) -> StdResult<GetMinMaxResponse> {
        let min_max = REQUIRED_MIN_MAX.load(deps.storage)?;
        Ok(GetMinMaxResponse {
            common_min: min_max.common_min,
            common_max: min_max.common_max,
            uncommon_min: min_max.uncommon_min,
            uncommon_max: min_max.uncommon_max,
            rare_min: min_max.rare_min,
            rare_max: min_max.rare_max,
            epic_min: min_max.epic_min,
            epic_max: min_max.epic_max,
            legendary_min: min_max.legendary_min,
            legendary_max: min_max.legendary_max,
        })
    }
}
