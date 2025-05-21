
use super::super::types::*;
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)] 
 

#[carbon(discriminator = "0xb617adf097ceb643")] 
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