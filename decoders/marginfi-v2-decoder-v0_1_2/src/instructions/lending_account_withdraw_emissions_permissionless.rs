

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x04ae7ccb2c319196")]
pub struct LendingAccountWithdrawEmissionsPermissionless{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct LendingAccountWithdrawEmissionsPermissionlessInstructionAccounts {
    pub group: solana_pubkey::Pubkey,
    pub marginfi_account: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
    pub emissions_mint: solana_pubkey::Pubkey,
    pub emissions_auth: solana_pubkey::Pubkey,
    pub emissions_vault: solana_pubkey::Pubkey,
    pub destination_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingAccountWithdrawEmissionsPermissionless {
    type ArrangedAccounts = LendingAccountWithdrawEmissionsPermissionlessInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            group,
            marginfi_account,
            bank,
            emissions_mint,
            emissions_auth,
            emissions_vault,
            destination_account,
            token_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(LendingAccountWithdrawEmissionsPermissionlessInstructionAccounts {
            group: group.pubkey,
            marginfi_account: marginfi_account.pubkey,
            bank: bank.pubkey,
            emissions_mint: emissions_mint.pubkey,
            emissions_auth: emissions_auth.pubkey,
            emissions_vault: emissions_vault.pubkey,
            destination_account: destination_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}