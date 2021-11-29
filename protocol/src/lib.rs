pub mod block_header;
mod errors;

// BlockHeaderParser parser node block to model block
pub struct BlockHeaderParser {
    netParams:,
}

impl BlockHeaderParser {
    // NewBlockHeaderParser new object
    pub fn new(netParams: * consensus.Params) -> Self {
        BlockHeaderParser { netParams: netParams }
    }

    // Parse get address transaction
    pub fn parse(block: types.Block, blockModel: &model.Block) -> Result<(), error> {
        let blockHash = block.Hash();
        rawBlockByte = block.MarshalText().unwarp();
        if err != nil {
            return Result::Err();
        }
        let rawBlockByte = match block.MarshalText() {
            Ok(()) => block.MarshalText(),
            Err(MarshalRawBlock) => return Result::Err(MarshalRawBlock),
        };

        blockModel.Header = &block_header {
            Height: block.Height,
            Hash: blockHash.String(),
            PreviousBlockHash: block.PreviousBlockHash.String(),
            TxMerkleRoot: block.TransactionsMerkleRoot.String(),
            TxCount: uint64(len(block.Transactions)),
            Size: uint64(len(rawBlockByte)),
            Timestamp: commonTime.Timestamp(time.Unix(int64(block.Timestamp / 1000), 0)),
            RawBlock: string(rawBlockByte),
        };

        Ok(())
    }
}


