
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0b6cd757f00942f1")]
pub struct EditStakedSettings{
    pub settings: StakedSettingsEditConfig,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct EditStakedSettingsInstructionAccounts {
    pub marginfi_group: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub staked_settings: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EditStakedSettings {
    type ArrangedAccounts = EditStakedSettingsInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            marginfi_group,
            admin,
            staked_settings,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(EditStakedSettingsInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            admin: admin.pubkey,
            staked_settings: staked_settings.pubkey,
        })
    }
}