

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xf5362904f3ca1f11")]
pub struct LendingAccountCloseBalance{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct LendingAccountCloseBalanceInstructionAccounts {
    pub group: solana_pubkey::Pubkey,
    pub marginfi_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingAccountCloseBalance {
    type ArrangedAccounts = LendingAccountCloseBalanceInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            group,
            marginfi_account,
            authority,
            bank,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(LendingAccountCloseBalanceInstructionAccounts {
            group: group.pubkey,
            marginfi_account: marginfi_account.pubkey,
            authority: authority.pubkey,
            bank: bank.pubkey,
        })
    }
}