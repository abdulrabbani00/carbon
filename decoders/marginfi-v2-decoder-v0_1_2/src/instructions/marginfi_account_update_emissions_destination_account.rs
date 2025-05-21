

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x49b9a2c96f1874b9")]
pub struct MarginfiAccountUpdateEmissionsDestinationAccount{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MarginfiAccountUpdateEmissionsDestinationAccountInstructionAccounts {
    pub marginfi_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub destination_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MarginfiAccountUpdateEmissionsDestinationAccount {
    type ArrangedAccounts = MarginfiAccountUpdateEmissionsDestinationAccountInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            marginfi_account,
            authority,
            destination_account,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(MarginfiAccountUpdateEmissionsDestinationAccountInstructionAccounts {
            marginfi_account: marginfi_account.pubkey,
            authority: authority.pubkey,
            destination_account: destination_account.pubkey,
        })
    }
}