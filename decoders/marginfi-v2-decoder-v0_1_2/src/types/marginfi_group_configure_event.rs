
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct MarginfiGroupConfigureEvent {
    pub header: GroupEventHeader,
    pub admin: solana_pubkey::Pubkey,
    pub flags: u64,
}
