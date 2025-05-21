

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xd152ffab7c154751")]
pub struct LendingPoolConfigureBankOracle{
    pub setup: u8,
    pub oracle: solana_pubkey::Pubkey,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolConfigureBankOracleInstructionAccounts {
    pub group: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingPoolConfigureBankOracle {
    type ArrangedAccounts = LendingPoolConfigureBankOracleInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            group,
            admin,
            bank,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(LendingPoolConfigureBankOracleInstructionAccounts {
            group: group.pubkey,
            admin: admin.pubkey,
            bank: bank.pubkey,
        })
    }
}