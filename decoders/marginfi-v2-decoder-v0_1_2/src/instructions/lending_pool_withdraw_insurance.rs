

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x6c3c3cf6684f9ff3")]
pub struct LendingPoolWithdrawInsurance{
    pub amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolWithdrawInsuranceInstructionAccounts {
    pub group: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub insurance_vault: solana_pubkey::Pubkey,
    pub insurance_vault_authority: solana_pubkey::Pubkey,
    pub dst_token_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingPoolWithdrawInsurance {
    type ArrangedAccounts = LendingPoolWithdrawInsuranceInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            group,
            bank,
            admin,
            insurance_vault,
            insurance_vault_authority,
            dst_token_account,
            token_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(LendingPoolWithdrawInsuranceInstructionAccounts {
            group: group.pubkey,
            bank: bank.pubkey,
            admin: admin.pubkey,
            insurance_vault: insurance_vault.pubkey,
            insurance_vault_authority: insurance_vault_authority.pubkey,
            dst_token_account: dst_token_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}