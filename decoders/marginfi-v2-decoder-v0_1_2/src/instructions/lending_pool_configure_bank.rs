
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x79ad9c285d9438ed")]
pub struct LendingPoolConfigureBank{
    pub bank_config_opt: BankConfigOpt,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolConfigureBankInstructionAccounts {
    pub group: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingPoolConfigureBank {
    type ArrangedAccounts = LendingPoolConfigureBankInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            group,
            admin,
            bank,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(LendingPoolConfigureBankInstructionAccounts {
            group: group.pubkey,
            admin: admin.pubkey,
            bank: bank.pubkey,
        })
    }
}