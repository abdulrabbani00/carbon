

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x99a23254b6c94ab3")]
pub struct SetNewAccountAuthority{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetNewAccountAuthorityInstructionAccounts {
    pub marginfi_account: solana_pubkey::Pubkey,
    pub group: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub new_authority: solana_pubkey::Pubkey,
    pub fee_payer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetNewAccountAuthority {
    type ArrangedAccounts = SetNewAccountAuthorityInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            marginfi_account,
            group,
            authority,
            new_authority,
            fee_payer,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(SetNewAccountAuthorityInstructionAccounts {
            marginfi_account: marginfi_account.pubkey,
            group: group.pubkey,
            authority: authority.pubkey,
            new_authority: new_authority.pubkey,
            fee_payer: fee_payer.pubkey,
        })
    }
}