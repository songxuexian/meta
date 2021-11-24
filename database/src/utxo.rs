use rbatis::crud_table;
use serde::{Serialize, Deserialize};

// pre-define tx types
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum UTXOType {
    // OrdinaryType represent ordinary utxo type
    OrdinaryType(u8),
    // CoinbaseType represent is coinbase uxto type
    CoinbaseType(u8),
    // VoteType represent vote utxo type
    VoteType(u8),
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
    pub utxo_type: Option<UTXOType>,
    pub amount: Option<u64>,
    pub raw_data: Option<String>,
    pub control_program: Option<String>,
    pub vote: Option<String>,
    pub created_at: Option<rbatis::DateTimeNative>,
    pub updated_at: Option<rbatis::DateTimeNative>,
}

