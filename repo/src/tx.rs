use rbatis::crud::CRUD;
use meta_database::tx::TransactionType;
use meta_database::tx::TxType::{VetoTxType};
use crate::connect::connect_db;
use crate::errors::RepoError;
use crate::errors::RepoError::RowsAffected;

pub async fn save_tx_type(tx_type: &TransactionType) -> Result<(), RepoError> {
    let rb = connect_db().await;
    let res = rb.save(tx_type, &[]).await.unwrap();
    if res.rows_affected != 1 {
        return Result::Err(RowsAffected("save tx type".to_string()));
    }

    Result::Ok(())
}

#[tokio::test]
pub async fn test_tx_type() {
    fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);

    // insert table
    let tx_type = &TransactionType {
        id: None,
        transaction_id: Some(11963),
        r#type: Some(VetoTxType),
        created_at: None,
        updated_at: None,
    };

    let res = save_tx_type(tx_type).await;
    match res {
        Ok(()) => println!("OK"),
        Err(error) => println!("Problem opening the file: {:?}", error)
    }
}
