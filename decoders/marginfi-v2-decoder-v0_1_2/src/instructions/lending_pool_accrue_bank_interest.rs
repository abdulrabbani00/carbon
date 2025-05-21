

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x6cc91e572f4161bc")]
pub struct LendingPoolAccrueBankInterest{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolAccrueBankInterestInstructionAccounts {
    pub group: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingPoolAccrueBankInterest {
    type ArrangedAccounts = LendingPoolAccrueBankInterestInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            group,
            bank,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(LendingPoolAccrueBankInterestInstructionAccounts {
            group: group.pubkey,
            bank: bank.pubkey,
        })
    }
}