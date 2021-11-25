use rbatis::crud::CRUD;
use meta_database::tx::TransactionType;
use meta_database::tx::TxType::{VetoTxType};
use crate::connect::connect_db;

#[tokio::test]
pub async fn test_tx_type() {
    fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
    let rb = connect_db().await;

    // insert table
    let tx_type = &TransactionType {
        id: None,
        transaction_id: Some(11963),
        r#type: Some(VetoTxType),
        created_at: None,
        updated_at: None,
    };
    rb.save(tx_type, &[]).await;

    // //delete table
    // rb.remove_by_column::<TransactionType, _>("transaction_id", &Some(11963)).await;
}
