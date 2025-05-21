 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use super::MarginfiDecoder; 
pub mod bank; 
pub mod fee_state; 
pub mod marginfi_account; 
pub mod marginfi_group; 
pub mod staked_settings; 

pub enum MarginfiAccount { 
        Bank(bank::Bank), 
        FeeState(fee_state::FeeState), 
        MarginfiAccount(marginfi_account::MarginfiAccount), 
        MarginfiGroup(marginfi_group::MarginfiGroup), 
        StakedSettings(staked_settings::StakedSettings), 
}


impl<'a> AccountDecoder<'a> for MarginfiDecoder { 
    type AccountType = MarginfiAccount;
     fn decode_account( &self, account: &solana_account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = bank::Bank::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MarginfiAccount::Bank(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = fee_state::FeeState::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MarginfiAccount::FeeState(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = marginfi_account::MarginfiAccount::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MarginfiAccount::MarginfiAccount(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = marginfi_group::MarginfiGroup::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MarginfiAccount::MarginfiGroup(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = staked_settings::StakedSettings::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MarginfiAccount::StakedSettings(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}