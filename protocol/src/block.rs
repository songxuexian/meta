use meta_database::tx::TxType;
use meta_database::utxo::UTXOType;
use std::collections::HashMap;

pub struct Block {
    pub header: Header,
    validators: HashMap<String, Validator>,
    // key = asset hash
    assets: HashMap<String, Asset>,
    // key  = control program
    pub addresses: HashMap<String, Address>,
    transactions: Vec<Transaction>,
    utxos: Vec<UTXO>,
    spend_utxo_hashes: Vec<String>,
    address_transactions: Vec<AddressTransaction>,
    balance: HashMap<String, Balance>, // key = asset + address
}

pub struct Address {
    pub script: String,
    pub address: String,
}

struct Asset {
    asset: String,
    issuance_program: String,
    definition: String,
    total_supply: String,
    symbol: String,
    name: String,
    decimal: u64,
}

struct Balance {
    script: String,
    asset: String,
    balance: String,
    total_received: String,
    total_sent: String,
}

struct Validator {
    pub_key: String,
    vote_num: i64,
    script: String,
    produced_block_count: i64,
}

struct Transaction {
    hash: String,
    time_range: u64,
    fee: u64,
    raw_data: String,
    types: Vec<TxType>,
}

struct AddressTransaction {
    script: String,
    asset: String,
    transaction_hash: String,
    amount: i64,
}

struct UTXO {
    hash: String,
    control_program: String,
    raw_data: String,
    amount: u64,
    address: String,
    asset: String,
    block_height: u64,
    valid_height: u64,
    r#type: UTXOType,
    vote: String,
    spend: bool,
}

pub struct Header {
    pub height: u64,
    pub hash: String,
    pub validator_key: String,
    pub previous_block_hash: String,
    pub tx_merkle_root: String,
    pub tx_count: u64,
    pub size: u64,
    pub timestamp: i64,
    pub raw_block: String,
}
