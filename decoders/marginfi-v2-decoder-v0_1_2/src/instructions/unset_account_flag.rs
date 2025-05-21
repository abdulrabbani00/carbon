

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x385138555c31ff46")]
pub struct UnsetAccountFlag{
    pub flag: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UnsetAccountFlagInstructionAccounts {
    pub group: solana_pubkey::Pubkey,
    pub marginfi_account: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UnsetAccountFlag {
    type ArrangedAccounts = UnsetAccountFlagInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            group,
            marginfi_account,
            admin,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(UnsetAccountFlagInstructionAccounts {
            group: group.pubkey,
            marginfi_account: marginfi_account.pubkey,
            admin: admin.pubkey,
        })
    }
}