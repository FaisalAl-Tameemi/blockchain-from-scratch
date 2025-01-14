use std::time::Duration;
use std::sync::Arc;

use blockchain_from_scratch::transaction::{pool::TransactionPool, transaction::Transaction};
use tokio::{task, time};
use tonic::{transport::Server, Request, Response, Status};
use rpc::blockchain_service_server::{BlockchainServiceServer, BlockchainService};
use rpc::{GetBlockchainHeightRequest, GetBlockchainHeightResponse, AddTransactionRequest, AddTransactionResponse};
use ::blockchain_from_scratch::DB;
use tokio::sync::RwLock;

pub mod rpc {
    tonic::include_proto!("rpc");
}

#[derive(Debug)]
pub struct NodeRPC {
    db: DB,
    transaction_pool: Arc<RwLock<TransactionPool>>,
}

#[tonic::async_trait]
impl BlockchainService for NodeRPC {
    async fn get_blockchain_height(
        &self,
        request: Request<GetBlockchainHeightRequest>,
    ) -> Result<Response<GetBlockchainHeightResponse>, Status> {
        Ok(Response::new(GetBlockchainHeightResponse {
            height: self.db.get_next_block_height().await.unwrap_or(0),
        }))
    }

    async fn add_transaction(&self, request: Request<AddTransactionRequest>) -> Result<Response<AddTransactionResponse>, Status> {
        let transaction = Transaction::default();
        self.transaction_pool.write().await.0.push(transaction);
        
        Ok(Response::new(AddTransactionResponse {
            success: true,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let node_rpc = NodeRPC {
        db: DB::new().await?,
        transaction_pool: Arc::new(RwLock::new(TransactionPool(vec![]))),
    };

    node_rpc.db.initialize().await?;
    Server::builder()
        .add_service(BlockchainServiceServer::new(node_rpc))
        .serve(addr)
        .await?;

    Ok(())
}
