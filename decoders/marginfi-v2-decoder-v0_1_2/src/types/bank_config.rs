
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct BankConfig {
    pub asset_weight_init: WrappedI80F48,
    pub asset_weight_maint: WrappedI80F48,
    pub liability_weight_init: WrappedI80F48,
    pub liability_weight_maint: WrappedI80F48,
    pub deposit_limit: u64,
    pub interest_rate_config: InterestRateConfig,
    pub operational_state: BankOperationalState,
    pub oracle_setup: OracleSetup,
    pub oracle_keys: [solana_pubkey::Pubkey; 5],
    pub pad0: [u8; 6],
    pub borrow_limit: u64,
    pub risk_tier: RiskTier,
    pub asset_tag: u8,
    pub pad1: [u8; 6],
    pub total_asset_value_init_limit: u64,
    pub oracle_max_age: u16,
    pub padding0: [u8; 6],
    pub padding1: [u8; 32],
}
