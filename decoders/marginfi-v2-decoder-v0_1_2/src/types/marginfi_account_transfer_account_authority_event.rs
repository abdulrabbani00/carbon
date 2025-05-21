
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct MarginfiAccountTransferAccountAuthorityEvent {
    pub header: AccountEventHeader,
    pub old_account_authority: solana_pubkey::Pubkey,
    pub new_account_authority: solana_pubkey::Pubkey,
}
