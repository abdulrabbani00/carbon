
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct MarginfiAccount {
    pub group: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub lending_account: LendingAccount,
    pub account_flags: u64,
    pub emissions_destination_account: solana_pubkey::Pubkey,
    pub health_cache: HealthCache,
    pub padding0: [u64; 21],
}
