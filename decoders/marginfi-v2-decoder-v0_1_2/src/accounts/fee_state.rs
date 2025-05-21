
use super::super::types::*;
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)] 
 

#[carbon(discriminator = "0x3fe01055c124ebdc")] 
pub struct FeeState {
        pub key: solana_pubkey::Pubkey,
        pub global_fee_admin: solana_pubkey::Pubkey,
        pub global_fee_wallet: solana_pubkey::Pubkey,
        pub placeholder0: u64,
        pub bank_init_flat_sol_fee: u32,
        pub bump_seed: u8,
        pub padding0: [u8; 4],
        pub padding1: [u8; 15],
        pub program_fee_fixed: WrappedI80F48,
        pub program_fee_rate: WrappedI80F48,
        pub reserved0: [u8; 32],
        #[serde(with = "serde_big_array::BigArray")]
        pub reserved1: [u8; 64], 
}