use web3::types::{Block, BlockId, H256};
use web3::{transports, Web3};

use crate::connect;
use crate::errors::NodeError;

trait ChainNode {
    type Output;
    async fn get_block_by_hash(&self,hash:String) -> Result<Self::Output, NodeError>;
}

pub struct Node {
    _url: String,
    web3: Web3<transports::Http>,
}

impl ChainNode for Node{
    type Output = Block<H256>;

    async fn  get_block_by_hash(&self, hash:String) -> Result<Self::Output, NodeError> {
        let hash = Hash::new(hash);
        match self.web3.eth().block(BlockId::Hash(hash)).await.unwrap() {
            Some(block) => return Ok(block),
            None => return Err(NodeError::NotFoundHash(hash.to_string())),
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
            Some(block) => return Ok(block),
            None => return Err(NodeError::NotFoundHash(hash.to_string())),
        }
    }
}
