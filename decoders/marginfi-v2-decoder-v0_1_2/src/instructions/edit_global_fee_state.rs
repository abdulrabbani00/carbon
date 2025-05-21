
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x343e23815d45a5ca")]
pub struct EditGlobalFeeState{
    pub admin: solana_pubkey::Pubkey,
    pub fee_wallet: solana_pubkey::Pubkey,
    pub bank_init_flat_sol_fee: u32,
    pub program_fee_fixed: WrappedI80F48,
    pub program_fee_rate: WrappedI80F48,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct EditGlobalFeeStateInstructionAccounts {
    pub global_fee_admin: solana_pubkey::Pubkey,
    pub fee_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EditGlobalFeeState {
    type ArrangedAccounts = EditGlobalFeeStateInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            global_fee_admin,
            fee_state,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(EditGlobalFeeStateInstructionAccounts {
            global_fee_admin: global_fee_admin.pubkey,
            fee_state: fee_state.pubkey,
        })
    }
}