use rbatis::crud_table;

#[crud_table]
#[derive(Clone, Debug)]
pub struct Balance {
    pub id: Option<u64>,
    pub address_id: Option<u64>,
    pub asset_id: Option<u64>,
    pub balance: Option<String>,
    pub total_received: Option<String>,
    pub total_sent: Option<String>,
    pub created_at: Option<rbatis::DateTimeNative>,
    pub updated_at: Option<rbatis::DateTimeNative>,
}
