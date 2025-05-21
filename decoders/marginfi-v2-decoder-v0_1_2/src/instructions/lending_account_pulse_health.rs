

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xba347561224a27fd")]
pub struct LendingAccountPulseHealth{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct LendingAccountPulseHealthInstructionAccounts {
    pub marginfi_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingAccountPulseHealth {
    type ArrangedAccounts = LendingAccountPulseHealthInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            marginfi_account,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(LendingAccountPulseHealthInstructionAccounts {
            marginfi_account: marginfi_account.pubkey,
        })
    }
}