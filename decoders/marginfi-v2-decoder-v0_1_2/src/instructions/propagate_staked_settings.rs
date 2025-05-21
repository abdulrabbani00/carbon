

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xd21e98458263deaa")]
pub struct PropagateStakedSettings{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct PropagateStakedSettingsInstructionAccounts {
    pub marginfi_group: solana_pubkey::Pubkey,
    pub staked_settings: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PropagateStakedSettings {
    type ArrangedAccounts = PropagateStakedSettingsInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            marginfi_group,
            staked_settings,
            bank,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(PropagateStakedSettingsInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            staked_settings: staked_settings.pubkey,
            bank: bank.pubkey,
        })
    }
}