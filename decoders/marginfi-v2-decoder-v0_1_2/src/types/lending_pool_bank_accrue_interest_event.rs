use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub struct LendingPoolBankAccrueInterestEvent {
    pub header: GroupEventHeader,
    pub bank: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub delta: u64,
    pub fees_collected: f64,
    pub insurance_collected: f64,
}
