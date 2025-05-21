

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x697cc96a9902089c")]
pub struct LendingAccountEndFlashloan{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct LendingAccountEndFlashloanInstructionAccounts {
    pub marginfi_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingAccountEndFlashloan {
    type ArrangedAccounts = LendingAccountEndFlashloanInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            marginfi_account,
            authority,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(LendingAccountEndFlashloanInstructionAccounts {
            marginfi_account: marginfi_account.pubkey,
            authority: authority.pubkey,
        })
    }
}