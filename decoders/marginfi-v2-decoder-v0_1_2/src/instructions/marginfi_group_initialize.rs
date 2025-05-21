

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xff43431a5e1f2214")]
pub struct MarginfiGroupInitialize{
    pub is_arena_group: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MarginfiGroupInitializeInstructionAccounts {
    pub marginfi_group: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub fee_state: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MarginfiGroupInitialize {
    type ArrangedAccounts = MarginfiGroupInitializeInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            marginfi_group,
            admin,
            fee_state,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(MarginfiGroupInitializeInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            admin: admin.pubkey,
            fee_state: fee_state.pubkey,
            system_program: system_program.pubkey,
        })
    }
}