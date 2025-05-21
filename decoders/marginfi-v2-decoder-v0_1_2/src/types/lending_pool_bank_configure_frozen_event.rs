
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct LendingPoolBankConfigureFrozenEvent {
    pub header: GroupEventHeader,
    pub bank: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub deposit_limit: u64,
    pub borrow_limit: u64,
}
