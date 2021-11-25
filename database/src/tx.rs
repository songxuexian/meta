use rbatis::crud_table;
use serde::{Serialize, Deserialize};
use serde_repr::*;

#[crud_table]
#[derive(Clone, Debug)]
pub struct Transaction {
    pub id: Option<u64>,
    pub block_id: Option<u64>,
    pub transaction_index: Option<u64>,
    pub hash: Option<String>,
    pub fee: Option<u64>,
    pub raw_data: Option<String>,
    pub created_at: Option<rbatis::DateTimeNative>,
    pub updated_at: Option<rbatis::DateTimeNative>,
}

// #[crud_table(table_name: "transaction_types" | table_columns: "id,transaction_id,type,created_at,updated_at")]
#[crud_table(table_name: "transaction_types")]
#[derive(Clone, Debug)]
pub struct TransactionType {
    pub id: Option<u64>,
    pub transaction_id: Option<u64>,
    pub r#type: Option<TxType>,
    pub created_at: Option<rbatis::DateTimeNative>,
    pub updated_at: Option<rbatis::DateTimeNative>,
}


// pre-define tx types
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Eq, Clone)]
#[repr(u8)]
pub enum TxType {
    OrdinaryTxType = 1,
    CoinBaseTxType,
    IssuanceIxType,
    VetoTxType,
    VoteTxType,
    CrossChainTxType,
    ChainTxType,
}




