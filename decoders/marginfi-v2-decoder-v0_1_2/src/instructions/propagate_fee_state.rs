

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x4003a6c28115659b")]
pub struct PropagateFeeState{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct PropagateFeeStateInstructionAccounts {
    pub fee_state: solana_pubkey::Pubkey,
    pub marginfi_group: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PropagateFeeState {
    type ArrangedAccounts = PropagateFeeStateInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            fee_state,
            marginfi_group,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(PropagateFeeStateInstructionAccounts {
            fee_state: fee_state.pubkey,
            marginfi_group: marginfi_group.pubkey,
        })
    }
}