use web3::types::{Block, BlockId, BlockNumber, H256, U64};
use web3::{transports, Web3};

use crate::connect;
use crate::errors::NodeError;
use async_trait::async_trait;

#[async_trait]
trait ChainNode {
    type Output;
    async fn get_block_by_hash(&self, hash: String) -> Result<Self::Output, NodeError>;
    async fn get_block_by_height(&self, height: u64) -> Result<Self::Output, NodeError>;
}

pub struct Node {
    _url: String,
    web3: Web3<transports::Http>,
}

#[async_trait]
impl ChainNode for Node {
    type Output = Block<H256>;

    async fn get_block_by_hash(&self, hash: String) -> Result<Self::Output, NodeError> {
        let hash = H256::from_slice(String::as_bytes(&hash));
        match self.web3.eth().block(BlockId::Hash(hash)).await.unwrap() {
            Some(block) => return Ok(block),
            None => return Err(NodeError::NotFoundHash(hash.to_string())),
        }
    }

    async fn get_block_by_height(&self, height: u64) -> Result<Self::Output, NodeError> {
        let block_number = BlockNumber::Number(U64::from(height));
        match self
            .web3
            .eth()
            .block(BlockId::Number(block_number))
            .await
            .unwrap()
        {
            Some(block) => return Ok(block),
            None => return Err(NodeError::NotFoundHeight(height)),
        }
    }
}

impl Node {
    pub fn new(url: &str) -> Self {
        Node {
            _url: url.to_string(),
            web3: connect::web3(url),
        }
    }

    pub async fn get_block_by_hash(&self, hash: H256) -> Result<Block<H256>, NodeError> {
        match self.web3.eth().block(BlockId::Hash(hash)).await.unwrap() {
            Some(block) => Ok(block),
            None => Err(NodeError::NotFoundHash(hash.to_string())),
        }
    }
}
