use rbatis::crud_table;

#[crud_table]
#[derive(Clone, Debug)]
pub struct Address {
    pub id: Option<u64>,
    pub script: Option<String>,
    pub address: Option<String>,
    pub created_at: Option<rbatis::DateTimeNative>,
    pub updated_at: Option<rbatis::DateTimeNative>,
}


#[crud_table]
#[derive(Clone, Debug)]
pub struct AddressTransaction {
    pub id: Option<u64>,
    pub address_id: Option<u64>,
    pub transaction_id: Option<u64>,
    pub asset_id: Option<u64>,
    pub amount: Option<i64>,
    pub created_at: Option<rbatis::DateTimeNative>,
    pub updated_at: Option<rbatis::DateTimeNative>,
}



