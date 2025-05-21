

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xc905d774e65c4b96")]
pub struct LendingPoolCollectBankFees{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolCollectBankFeesInstructionAccounts {
    pub group: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
    pub liquidity_vault_authority: solana_pubkey::Pubkey,
    pub liquidity_vault: solana_pubkey::Pubkey,
    pub insurance_vault: solana_pubkey::Pubkey,
    pub fee_vault: solana_pubkey::Pubkey,
    pub fee_state: solana_pubkey::Pubkey,
    pub fee_ata: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingPoolCollectBankFees {
    type ArrangedAccounts = LendingPoolCollectBankFeesInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            group,
            bank,
            liquidity_vault_authority,
            liquidity_vault,
            insurance_vault,
            fee_vault,
            fee_state,
            fee_ata,
            token_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(LendingPoolCollectBankFeesInstructionAccounts {
            group: group.pubkey,
            bank: bank.pubkey,
            liquidity_vault_authority: liquidity_vault_authority.pubkey,
            liquidity_vault: liquidity_vault.pubkey,
            insurance_vault: insurance_vault.pubkey,
            fee_vault: fee_vault.pubkey,
            fee_state: fee_state.pubkey,
            fee_ata: fee_ata.pubkey,
            token_program: token_program.pubkey,
        })
    }
}