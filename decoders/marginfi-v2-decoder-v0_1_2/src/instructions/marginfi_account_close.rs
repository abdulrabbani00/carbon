

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xbadd5d223261c2f1")]
pub struct MarginfiAccountClose{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MarginfiAccountCloseInstructionAccounts {
    pub marginfi_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub fee_payer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MarginfiAccountClose {
    type ArrangedAccounts = MarginfiAccountCloseInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            marginfi_account,
            authority,
            fee_payer,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(MarginfiAccountCloseInstructionAccounts {
            marginfi_account: marginfi_account.pubkey,
            authority: authority.pubkey,
            fee_payer: fee_payer.pubkey,
        })
    }
}