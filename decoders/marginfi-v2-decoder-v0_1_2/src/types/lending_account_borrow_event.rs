
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct LendingAccountBorrowEvent {
    pub header: AccountEventHeader,
    pub bank: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub amount: u64,
}
