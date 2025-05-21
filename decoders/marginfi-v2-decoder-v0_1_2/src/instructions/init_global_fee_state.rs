
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x5230f73bdc6de72c")]
pub struct InitGlobalFeeState{
    pub admin: solana_pubkey::Pubkey,
    pub fee_wallet: solana_pubkey::Pubkey,
    pub bank_init_flat_sol_fee: u32,
    pub program_fee_fixed: WrappedI80F48,
    pub program_fee_rate: WrappedI80F48,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitGlobalFeeStateInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub fee_state: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitGlobalFeeState {
    type ArrangedAccounts = InitGlobalFeeStateInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            payer,
            fee_state,
            rent,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(InitGlobalFeeStateInstructionAccounts {
            payer: payer.pubkey,
            fee_state: fee_state.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}