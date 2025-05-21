
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct HealthCache {
    pub asset_value: WrappedI80F48,
    pub liability_value: WrappedI80F48,
    pub timestamp: i64,
    pub flags: u64,
    pub prices: [WrappedI80F48; 16],
}
