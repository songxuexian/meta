use crate::block::{Block, Header};
use crate::errors::ProtocolError;

// BlockHeaderParser parser node block to model block
pub struct BlockHeaderParser {
    net_params: String,
}

impl BlockHeaderParser {
    // NewBlockHeaderParser new object
    pub fn new(net_params: String) -> Self {
        BlockHeaderParser { net_params }
    }

    // Parse get address transaction
    pub fn parse(block: impl ChainBlock, block_model: &mut Block) -> Result<(), ProtocolError> {
        let raw_block_byte = match block.marshal_text() {
            Ok(text) => text,
            Err(error) => return Result::Err(error),
        };

        block_model.header = Header {
            height: block.height(),
            hash: block.hash(),
            validator_key: "".to_string(),
            previous_block_hash: block.previous_block_hash(),
            tx_merkle_root: block.transactions_merkle_root(),
            tx_count: block.transactions().len() as u64,
            size: raw_block_byte.len() as u64,
            // convert ms to s
            timestamp: block.timestamp() / 1000,
            raw_block: std::str::from_utf8(&raw_block_byte)
                .unwrap()
                .parse()
                .unwrap(),
        };

        Ok(())
    }
}

pub trait ChainBlock {
    fn hash(&self) -> String;
    fn height(&self) -> u64;
    fn marshal_text(&self) -> Result<Vec<u8>, ProtocolError>;
    fn previous_block_hash(&self) -> String;
    fn transactions_merkle_root(&self) -> String;
    fn timestamp(&self) -> i64;
    // todo need to change () to real type
    fn transactions(&self) -> Vec<Box<dyn ChainTx>>;
}

pub trait ChainTx {
    fn inputs(&self) -> Vec<Box<dyn Input>>;
    fn outputs(&self) -> Vec<Box<dyn Output>>;
}

pub trait Input {
    fn control_program(&self) -> String;
}

pub trait Output {
    fn control_program(&self) -> String;
}
