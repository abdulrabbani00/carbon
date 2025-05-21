
use super::super::types::*;
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)] 
 

#[carbon(discriminator = "0x8e31a6f2324261bc")] 
pub struct Bank {
        pub mint: solana_pubkey::Pubkey,
        pub mint_decimals: u8,
        pub group: solana_pubkey::Pubkey,
        pub pad0: [u8; 7],
        pub asset_share_value: WrappedI80F48,
        pub liability_share_value: WrappedI80F48,
        pub liquidity_vault: solana_pubkey::Pubkey,
        pub liquidity_vault_bump: u8,
        pub liquidity_vault_authority_bump: u8,
        pub insurance_vault: solana_pubkey::Pubkey,
        pub insurance_vault_bump: u8,
        pub insurance_vault_authority_bump: u8,
        pub pad1: [u8; 4],
        pub collected_insurance_fees_outstanding: WrappedI80F48,
        pub fee_vault: solana_pubkey::Pubkey,
        pub fee_vault_bump: u8,
        pub fee_vault_authority_bump: u8,
        pub pad2: [u8; 6],
        pub collected_group_fees_outstanding: WrappedI80F48,
        pub total_liability_shares: WrappedI80F48,
        pub total_asset_shares: WrappedI80F48,
        pub last_update: i64,
        pub config: BankConfig,
        pub flags: u64,
        pub emissions_rate: u64,
        pub emissions_remaining: WrappedI80F48,
        pub emissions_mint: solana_pubkey::Pubkey,
        pub collected_program_fees_outstanding: WrappedI80F48,
        pub padding_0: [[u64; 2]; 27],
        pub padding_1: [[u64; 2]; 32], 
}