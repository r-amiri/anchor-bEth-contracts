use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Decimal, HumanAddr, Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub owner: HumanAddr,
    pub reward_denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    ////////////////////
    /// Owner's operations
    ///////////////////

    /// Set the token contract addess after initialization
    PostInitialize { token_contract: HumanAddr },

    /// Update contract configuration, currently only owner is updatable
    UpdateConfig { owner: HumanAddr },

    ////////////////////
    /// bAsset's operations
    ///////////////////

    /// Increase user staking balance
    /// Withdraw rewards to pending rewards
    /// Set current reward index to global index
    IncreaseBalance { address: HumanAddr, amount: Uint128 },
    /// Decrease user staking balance
    /// Withdraw rewards to pending rewards
    /// Set current reward index to global index
    DecreaseBalance { address: HumanAddr, amount: Uint128 },

    ////////////////////
    /// User's operations
    ///////////////////

    /// return the accrued reward in uusd to the user.
    ClaimRewards { recipient: Option<HumanAddr> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    State {},
    AccruedRewards {
        address: HumanAddr,
    },
    Holder {
        address: HumanAddr,
    },
    Holders {
        start_after: Option<HumanAddr>,
        limit: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner: HumanAddr,
    pub reward_denom: String,
    pub token_contract: Option<HumanAddr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StateResponse {
    pub global_index: Decimal,
    pub total_balance: Uint128,
    pub prev_reward_balance: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AccruedRewardsResponse {
    pub rewards: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct HolderResponse {
    pub address: HumanAddr,
    pub balance: Uint128,
    pub index: Decimal,
    pub pending_rewards: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct HoldersResponse {
    pub holders: Vec<HolderResponse>,
}
