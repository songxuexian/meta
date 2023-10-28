use rbatis::crud_table;

#[crud_table]
#[derive(Clone, Debug)]
pub struct Asset {
    pub id: Option<u64>,
    pub asset: Option<String>,
    pub issuance_program: Option<String>,
    pub definition: Option<String>,
    pub total_supply: Option<String>,
    pub symbol: Option<String>,
    pub name: Option<String>,
    pub decimal: Option<u32>,
    pub created_at: Option<rbatis::DateTimeNative>,
    pub updated_at: Option<rbatis::DateTimeNative>,
}
