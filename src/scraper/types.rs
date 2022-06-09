use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GenesisIntelligence {
    pub block_time: i64,
    pub mint_price: Option<u64>,
    pub genesis_transaction_signature: String,
    pub genesis_minter: Option<String>,
    pub mint: String,
}
