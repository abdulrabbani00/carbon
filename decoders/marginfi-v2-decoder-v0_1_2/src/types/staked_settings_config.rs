
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct StakedSettingsConfig {
    pub oracle: solana_pubkey::Pubkey,
    pub asset_weight_init: WrappedI80F48,
    pub asset_weight_maint: WrappedI80F48,
    pub deposit_limit: u64,
    pub total_asset_value_init_limit: u64,
    pub oracle_max_age: u16,
    pub risk_tier: RiskTier,
}
