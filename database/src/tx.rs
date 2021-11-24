use rbatis::crud_table;
use serde::{Serialize, Deserialize};

#[crud_table]
#[derive(Clone, Debug)]
pub struct Transaction {
    pub id: Option<String>,
    pub block_id: Option<String>,
    pub transaction_index: Option<u64>,
    pub hash: Option<String>,
    pub fee: Option<u64>,
    pub raw_data: Option<String>,
    pub created_at: Option<rbatis::DateTimeNative>,
    pub updated_at: Option<rbatis::DateTimeNative>,
}

#[crud_table]
#[derive(Clone, Debug)]
pub struct TransactionType {
    pub id: Option<String>,
    pub transaction_id: Option<String>,
    pub tx_type: Option<TxType>,
    pub created_at: Option<rbatis::DateTimeNative>,
    pub updated_at: Option<rbatis::DateTimeNative>,
}


// pre-define tx types
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum TxType {
    // OrdinaryTxType tx is ordinary
    OrdinaryTxType,
    CoinBaseTxType,
    // IssuanceIxType tx is issuance
    IssuanceIxType,
    // VetoTxType tx is veto type
    VetoTxType,
    // VoteTxType tx is vote type
    VoteTxType,
    // CrossChainTxType tx is cross chain
    CrossChainTxType,
    // ChainTxType tx is chain type
    ChainTxType,
}




