
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct StakedSettingsEditConfig {
    pub oracle: Option<solana_pubkey::Pubkey>,
    pub asset_weight_init: Option<WrappedI80F48>,
    pub asset_weight_maint: Option<WrappedI80F48>,
    pub deposit_limit: Option<u64>,
    pub total_asset_value_init_limit: Option<u64>,
    pub oracle_max_age: Option<u16>,
    pub risk_tier: Option<RiskTier>,
}
