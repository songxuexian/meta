use rbatis::crud_table;
use serde::{Deserialize, Serialize};

// pre-define tx types
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum UTXOType {
    // OrdinaryType represent ordinary type
    OrdinaryType,
    // CoinbaseType represent is coinbase type
    CoinbaseType,
    // VoteType represent vote type
    VoteType,
}

#[crud_table]
#[derive(Clone, Debug)]
pub struct Utxo {
    pub id: Option<u64>,
    pub address_id: Option<u64>,
    pub asset_id: Option<u64>,
    pub hash: Option<String>,
    pub block_height: Option<u64>,
    pub valid_height: Option<u64>,
    pub is_spend: Option<bool>,
    pub r#type: Option<UTXOType>,
    pub amount: Option<u64>,
    pub raw_data: Option<String>,
    pub control_program: Option<String>,
    pub vote: Option<String>,
    pub created_at: Option<rbatis::DateTimeNative>,
    pub updated_at: Option<rbatis::DateTimeNative>,
}
