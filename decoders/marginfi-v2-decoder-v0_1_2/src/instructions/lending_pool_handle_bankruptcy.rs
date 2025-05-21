

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xa20b388b5a8046ad")]
pub struct LendingPoolHandleBankruptcy{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolHandleBankruptcyInstructionAccounts {
    pub group: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
    pub marginfi_account: solana_pubkey::Pubkey,
    pub liquidity_vault: solana_pubkey::Pubkey,
    pub insurance_vault: solana_pubkey::Pubkey,
    pub insurance_vault_authority: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingPoolHandleBankruptcy {
    type ArrangedAccounts = LendingPoolHandleBankruptcyInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            group,
            signer,
            bank,
            marginfi_account,
            liquidity_vault,
            insurance_vault,
            insurance_vault_authority,
            token_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(LendingPoolHandleBankruptcyInstructionAccounts {
            group: group.pubkey,
            signer: signer.pubkey,
            bank: bank.pubkey,
            marginfi_account: marginfi_account.pubkey,
            liquidity_vault: liquidity_vault.pubkey,
            insurance_vault: insurance_vault.pubkey,
            insurance_vault_authority: insurance_vault_authority.pubkey,
            token_program: token_program.pubkey,
        })
    }
}