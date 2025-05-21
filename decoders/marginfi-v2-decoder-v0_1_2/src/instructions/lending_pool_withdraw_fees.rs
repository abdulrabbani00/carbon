

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x5c8cd7feaa0053ae")]
pub struct LendingPoolWithdrawFees{
    pub amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolWithdrawFeesInstructionAccounts {
    pub group: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub fee_vault: solana_pubkey::Pubkey,
    pub fee_vault_authority: solana_pubkey::Pubkey,
    pub dst_token_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingPoolWithdrawFees {
    type ArrangedAccounts = LendingPoolWithdrawFeesInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            group,
            bank,
            admin,
            fee_vault,
            fee_vault_authority,
            dst_token_account,
            token_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(LendingPoolWithdrawFeesInstructionAccounts {
            group: group.pubkey,
            bank: bank.pubkey,
            admin: admin.pubkey,
            fee_vault: fee_vault.pubkey,
            fee_vault_authority: fee_vault_authority.pubkey,
            dst_token_account: dst_token_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}