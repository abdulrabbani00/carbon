

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe7cd42f2dc579126")]
pub struct ConfigGroupFee{
    pub enable_program_fee: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ConfigGroupFeeInstructionAccounts {
    pub marginfi_group: solana_pubkey::Pubkey,
    pub global_fee_admin: solana_pubkey::Pubkey,
    pub fee_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ConfigGroupFee {
    type ArrangedAccounts = ConfigGroupFeeInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            marginfi_group,
            global_fee_admin,
            fee_state,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(ConfigGroupFeeInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            global_fee_admin: global_fee_admin.pubkey,
            fee_state: fee_state.pubkey,
        })
    }
}