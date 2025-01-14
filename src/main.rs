use blockchain_from_scratch::{common::db::DbBlock, DB};

#[tokio::main]
async fn main() {
    let db = DB::new().await.unwrap();
    db.initialize().await.unwrap();
    let height = db.get_next_block_height().await.unwrap();
    println!("Next block height: {}", height);
    let block = DbBlock {
        height,
        hash: String::from(""),
    };
    db.insert_block(block).await.unwrap();
    let height = db.get_next_block_height().await.unwrap();
    println!("Next block height: {}", height);
}
