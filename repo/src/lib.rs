pub mod connect;
pub mod tx;
mod errors;

use rbatis::crud::CRUD;
use meta_database::block::Block;
use tokio::test;
use crate::connect::connect_db;

#[tokio::test]
pub async fn test_postgres_uuid() {
    fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
    let rb = connect_db().await;

    // let uuid = Uuid::from_str("df07fea2-b819-4e05-b86d-dfc15a5f52a9").unwrap();
    //create table
    // rb.exec("CREATE TABLE biz_uuid( id uuid, name VARCHAR, PRIMARY KEY(id));").await;

    //insert table
    // let block = &Block {
    //     id: None,
    //     validator_id: Some(1),
    //     height: Some(11958),
    //     hash: Some("881ee8c6b3f65123103db3be8a73324affe52ca6f85cbdaef203702a10e6a21b".to_string()),
    //     previous_block_hash: Some("8ee6db34c85490d41daa9ffa536a15af526e853407c66caeeb93df8e77f4798a".to_string()),
    //     tx_merkle_root: Some("8dd6db34c85490d41daa9ffa536a15af526e853407c66caeeb93df8e77f4798a".to_string()),
    //     tx_count: Some(1),
    //     size: Some(19),
    //     timestamp: Some(12812831283),
    //     created_at: None,
    //     updated_at: None,
    // };
    // rb.save(block, &[]).await;

    //update table
    // rb.update_by_column::<Block, _>("id", &Block { id: Some(1), created_at: Some("test_updated".to_string()) }).await;

    //query table
    let data: Block = rb.fetch_by_column("id", &Some(11964)).await.unwrap();
    println!("{:?}", data);

    //delete table
    rb.remove_by_column::<Block, _>("id", &Some(11964)).await;
}
