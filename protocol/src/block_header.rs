pub struct BlockHeader {
    Height: u64,
    Hash: str,
    ValidatorKey: str,
    PreviousBlockHash: str,
    TxMerkleRoot: str,
    TxCount: u64,
    Size: u64,
    Timestamp: i64,
    RawBlock: str,
}
