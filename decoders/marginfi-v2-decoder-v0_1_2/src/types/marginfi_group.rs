
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct MarginfiGroup {
    pub admin: solana_pubkey::Pubkey,
    pub group_flags: u64,
    pub fee_state_cache: FeeStateCache,
    pub banks: u16,
    pub pad0: [u8; 6],
    pub padding_0: [[u64; 2]; 26],
    pub padding_1: [[u64; 2]; 32],
    pub padding_3: u64,
    pub padding_4: u64,
}
