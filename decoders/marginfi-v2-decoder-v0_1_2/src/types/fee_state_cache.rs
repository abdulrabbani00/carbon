
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct FeeStateCache {
    pub global_fee_wallet: solana_pubkey::Pubkey,
    pub program_fee_fixed: WrappedI80F48,
    pub program_fee_rate: WrappedI80F48,
}
