

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x4fd1acb1de33ad97")]
pub struct LendingAccountRepay{
    pub amount: u64,
    pub repay_all: Option<bool>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct LendingAccountRepayInstructionAccounts {
    pub group: solana_pubkey::Pubkey,
    pub marginfi_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
    pub signer_token_account: solana_pubkey::Pubkey,
    pub liquidity_vault: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingAccountRepay {
    type ArrangedAccounts = LendingAccountRepayInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            group,
            marginfi_account,
            authority,
            bank,
            signer_token_account,
            liquidity_vault,
            token_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(LendingAccountRepayInstructionAccounts {
            group: group.pubkey,
            marginfi_account: marginfi_account.pubkey,
            authority: authority.pubkey,
            bank: bank.pubkey,
            signer_token_account: signer_token_account.pubkey,
            liquidity_vault: liquidity_vault.pubkey,
            token_program: token_program.pubkey,
        })
    }
}