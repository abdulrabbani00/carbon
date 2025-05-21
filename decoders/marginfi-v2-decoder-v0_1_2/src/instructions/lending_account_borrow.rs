

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x047e74353005d41f")]
pub struct LendingAccountBorrow{
    pub amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct LendingAccountBorrowInstructionAccounts {
    pub group: solana_pubkey::Pubkey,
    pub marginfi_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
    pub destination_token_account: solana_pubkey::Pubkey,
    pub bank_liquidity_vault_authority: solana_pubkey::Pubkey,
    pub liquidity_vault: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingAccountBorrow {
    type ArrangedAccounts = LendingAccountBorrowInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            group,
            marginfi_account,
            authority,
            bank,
            destination_token_account,
            bank_liquidity_vault_authority,
            liquidity_vault,
            token_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(LendingAccountBorrowInstructionAccounts {
            group: group.pubkey,
            marginfi_account: marginfi_account.pubkey,
            authority: authority.pubkey,
            bank: bank.pubkey,
            destination_token_account: destination_token_account.pubkey,
            bank_liquidity_vault_authority: bank_liquidity_vault_authority.pubkey,
            liquidity_vault: liquidity_vault.pubkey,
            token_program: token_program.pubkey,
        })
    }
}