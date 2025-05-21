use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub struct LendingPoolBankHandleBankruptcyEvent {
    pub header: AccountEventHeader,
    pub bank: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub bad_debt: f64,
    pub covered_amount: f64,
    pub socialized_amount: f64,
}
