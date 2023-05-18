use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128, Uint64};
use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub drgn_contract: Addr,
    pub allowed_cw20: Addr,
    pub allowed_operators: Vec<String>,
    pub random_key: i32,
    pub drgn_rac: Uint128,
    pub season: String,
    pub dragon: Addr,
    pub updated_dragon: String,
    pub egg_minter: String,
    pub drgn_recipient: String,
    pub cw20_recipient: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),
    EditState {
        owner: String,
        drgn_contract: Addr,
        allowed_cw20: Addr,
        allowed_operators: Vec<String>,
        random_key: i32,
        drgn_rac: Uint128,
        season: String,
    },
    EditContracts {
        dragon: Addr,
        updated_dragon: String,
        egg_minter: String,
        drgn_recipient: String,
        cw20_recipient: String,
    },
    EditStats {
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
    },
    EditMinMax {
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
    },
    UpgradeAdmin {
        owner: String,
        id_1: Uint64,
        id_2: Uint64,
        id_3: Uint64,
        rarity: String,
    },
    EditOldMinterState(MinterEditState),
    EditOldMinterContracts(MinterEditContracts),
    OldMinterDragonBirth(MinterDragonBirth),
}

#[cw_serde]
pub enum ReceiveMsg {
    Upgrade {
        owner: String,
        id_1: Uint64,
        id_2: Uint64,
        id_3: Uint64,
        rarity: String,
        res: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetStateResponse)]
    GetState {},
    #[returns(GetStatsResponse)]
    GetStats {},
    #[returns(GetMinMaxResponse)]
    GetMinMax {},
}

#[cw_serde]
pub struct GetStateResponse {
    pub owner: String,
    pub drgn_contract: Addr,
    pub allowed_cw20: Addr,
    pub allowed_operators: Vec<String>,
    pub random_key: i32,
    pub drgn_rac: Uint128,
    pub season: String,
    pub dragon: Addr,
    pub updated_dragon: String,
    pub egg_minter: String,
    pub drgn_recipient: String,
    pub cw20_recipient: String,
}

#[cw_serde]
pub struct GetStatsResponse {
    pub common_reward: Uint64,
    pub common_ovulation: Uint64,
    pub uncommon_reward: Uint64,
    pub uncommon_ovulation: Uint64,
    pub rare_reward: Uint64,
    pub rare_ovulation: Uint64,
    pub epic_reward: Uint64,
    pub epic_ovulation: Uint64,
    pub legendary_reward: Uint64,
    pub legendary_ovulation: Uint64,
}

#[cw_serde]
pub struct GetMinMaxResponse {
    pub common_min: Uint128,
    pub common_max: Uint128,
    pub uncommon_min: Uint128,
    pub uncommon_max: Uint128,
    pub rare_min: Uint128,
    pub rare_max: Uint128,
    pub epic_min: Uint128,
    pub epic_max: Uint128,
    pub legendary_min: Uint128,
    pub legendary_max: Uint128,
}

// QUERY DRAGON INFO FROM CURRENT DRAGON
#[cw_serde]
pub struct DragonInfoMsg {
    pub id: Uint64,
}

#[cw_serde]
#[allow(non_snake_case)]
pub struct GetDragonInfoMsg {
    pub DragonInfo: DragonInfoMsg,
}

#[cw_serde]
pub struct DragonResponse {
    pub owner: String,
    pub token_id: String,
    pub kind: String,
    pub ovulation_period: u64,
    pub hatch: Uint64,
    pub daily_income: String,
    pub is_staked: bool,
    pub stake_start_time: Uint64,
    pub reward_start_time: Uint64,
    pub unstaking_start_time: Uint64,
    pub unstaking_process: bool,
    pub reward_end_time: Uint64,
}

//EXECUTE DRAGON BIRTH MSG FROM OLD MINTER
#[cw_serde]
pub struct MinterDragonBirth {
    pub dragon_birth: MinterDragonBirthMsg,
}

#[cw_serde]
pub struct MinterDragonBirthMsg {
    id: String,
    owner: String,
}

//EXECUTE EDIT OLD MINTER STATE
#[cw_serde]
pub struct MinterEditState {
    pub edit_state: MinterEditStateMsg,
}

#[cw_serde]
pub struct MinterEditStateMsg {
    new_owner: String,
    base_price: Uint128,
    random_key: i32,
    hatch_price: Uint128,
    egg_sale_size: Uint64,
    allowed_cw20: Addr,
}

//EXECUTE EDIT OLD MINTER CONTRACTS
#[cw_serde]
pub struct MinterEditContracts {
    pub edit_contracts: EditContractsMsg,
}

#[cw_serde]
pub struct EditContractsMsg {
    egg: String,
    dragon: String,
    recipient: String,
    multisig: String,
}
