

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x7fbb7922bba7ee66")]
pub struct LendingPoolAddBankPermissionless{
    pub bank_seed: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolAddBankPermissionlessInstructionAccounts {
    pub marginfi_group: solana_pubkey::Pubkey,
    pub staked_settings: solana_pubkey::Pubkey,
    pub fee_payer: solana_pubkey::Pubkey,
    pub bank_mint: solana_pubkey::Pubkey,
    pub sol_pool: solana_pubkey::Pubkey,
    pub stake_pool: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
    pub liquidity_vault_authority: solana_pubkey::Pubkey,
    pub liquidity_vault: solana_pubkey::Pubkey,
    pub insurance_vault_authority: solana_pubkey::Pubkey,
    pub insurance_vault: solana_pubkey::Pubkey,
    pub fee_vault_authority: solana_pubkey::Pubkey,
    pub fee_vault: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingPoolAddBankPermissionless {
    type ArrangedAccounts = LendingPoolAddBankPermissionlessInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            marginfi_group,
            staked_settings,
            fee_payer,
            bank_mint,
            sol_pool,
            stake_pool,
            bank,
            liquidity_vault_authority,
            liquidity_vault,
            insurance_vault_authority,
            insurance_vault,
            fee_vault_authority,
            fee_vault,
            rent,
            token_program,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(LendingPoolAddBankPermissionlessInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            staked_settings: staked_settings.pubkey,
            fee_payer: fee_payer.pubkey,
            bank_mint: bank_mint.pubkey,
            sol_pool: sol_pool.pubkey,
            stake_pool: stake_pool.pubkey,
            bank: bank.pubkey,
            liquidity_vault_authority: liquidity_vault_authority.pubkey,
            liquidity_vault: liquidity_vault.pubkey,
            insurance_vault_authority: insurance_vault_authority.pubkey,
            insurance_vault: insurance_vault.pubkey,
            fee_vault_authority: fee_vault_authority.pubkey,
            fee_vault: fee_vault.pubkey,
            rent: rent.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}