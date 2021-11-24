use rbatis::crud_table;

#[crud_table(table_name: blocks)]
#[derive(Clone, Debug)]
pub struct Block {
    pub id: Option<u64>,
    pub validator_id: Option<u64>,
    pub height: Option<u64>,
    pub hash: Option<String>,
    pub previous_block_hash: Option<String>,
    pub tx_merkle_root: Option<String>,
    pub tx_count: Option<u32>,
    pub size: Option<u32>,
    pub timestamp: Option<i64>,
    pub created_at: Option<rbatis::DateTimeNative>,
    pub updated_at: Option<rbatis::DateTimeNative>,
}
