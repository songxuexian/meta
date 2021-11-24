use rbatis::crud_table;

#[crud_table]
#[derive(Clone, Debug)]
pub struct Address {
    pub id: Option<String>,
    pub script: Option<String>,
    pub address: Option<String>,
    pub created_at: Option<rbatis::DateTimeNative>,
    pub updated_at: Option<rbatis::DateTimeNative>,
}


#[crud_table]
#[derive(Clone, Debug)]
pub struct AddressTransaction {
    pub id: Option<String>,
    pub address_id: Option<String>,
    pub transaction_id: Option<String>,
    pub asset_id: Option<String>,
    pub amount: Option<i64>,
    pub created_at: Option<rbatis::DateTimeNative>,
    pub updated_at: Option<rbatis::DateTimeNative>,
}



