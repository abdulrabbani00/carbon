
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x3423952c45564550")]
pub struct InitStakedSettings{
    pub settings: StakedSettingsConfig,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitStakedSettingsInstructionAccounts {
    pub marginfi_group: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub fee_payer: solana_pubkey::Pubkey,
    pub staked_settings: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitStakedSettings {
    type ArrangedAccounts = InitStakedSettingsInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            marginfi_group,
            admin,
            fee_payer,
            staked_settings,
            rent,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(InitStakedSettingsInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            admin: admin.pubkey,
            fee_payer: fee_payer.pubkey,
            staked_settings: staked_settings.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}