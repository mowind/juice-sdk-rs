use crate::types::{Address, Bytes, H256, U256, U64};
use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

/// The block header type returned from RPC calls.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockHeader {
    /// Hash of the parent
    pub parent_hash: H256,
    /// Miner's address
    #[serde(rename = "miner")]
    pub coinbase: Address,
    /// State root hash
    #[serde(rename = "stateRoot")]
    pub root: H256,
    /// Transactions root hash
    #[serde(rename = "transactionsRoot")]
    pub transaction_root: H256,
    /// Transactions receipts root hash
    #[serde(rename = "receiptsRoot")]
    pub receipts_root: H256,
    /// Block number. None if pending.
    pub number: Option<U64>,
    /// Gas limit
    #[serde(rename = "gasLimit")]
    pub gas_limit: U256,
    /// Gas used
    #[serde(rename = "gasUsed")]
    pub gas_used: U256,
    /// Timestamp
    pub timestamp: U256,
    /// Extra data
    #[serde(rename = "extraData")]
    pub extra: Bytes,
    /// Nonce
    pub nonce: Bytes,
    /// Hash of the block
    pub hash: H256,
}

/// The block type returned from RPC calls.
/// This is generic over a `TX` type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block<TX> {
    /// Hash of the parent
    #[serde(rename = "parentHash")]
    pub parent_hash: H256,
    /// Miner's address
    #[serde(rename = "miner")]
    pub coinbase: Address,
    /// State root hash
    #[serde(rename = "stateRoot")]
    pub root: H256,
    /// Transactions root hash
    #[serde(rename = "transactionsRoot")]
    pub transaction_root: H256,
    /// Transactions receipts root hash
    #[serde(rename = "receiptsRoot")]
    pub receipts_root: H256,
    /// Block number. None if pending.
    pub number: Option<U64>,
    /// Gas limit
    #[serde(rename = "gasLimit")]
    pub gas_limit: U256,
    /// Gas used
    #[serde(rename = "gasUsed")]
    pub gas_used: U256,
    /// Timestamp
    pub timestamp: U256,
    /// Extra data
    #[serde(rename = "extraData")]
    pub extra: Bytes,
    /// Nonce
    pub nonce: Bytes,
    /// Hash of the block
    pub hash: H256,
    /// Transactions
    pub transactions: Vec<TX>,
}

/// Block number
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockNumber {
    /// Latest block
    Latest,
    /// Earliest block (genesis)
    Earilest,
    /// Pending block (not yet part of blockchain)
    Pending,
    /// Block by number from canon chain
    Number(U64),
}

impl<T: Into<U64>> From<T> for BlockNumber {
    fn from(num: T) -> Self {
        BlockNumber::Number(num.into())
    }
}

impl Serialize for BlockNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            BlockNumber::Number(ref x) => serializer.serialize_str(&format!("0x{:x}", x)),
            BlockNumber::Latest => serializer.serialize_str("latest"),
            BlockNumber::Earilest => serializer.serialize_str("earliest"),
            BlockNumber::Pending => serializer.serialize_str("pending"),
        }
    }
}

/// Block identifier
#[derive(Debug, Clone, PartialEq)]
pub enum BlockId {
    /// By hash
    Hash(H256),
    /// By number
    Number(BlockNumber),
}

impl Serialize for BlockId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            BlockId::Hash(ref x) => {
                let mut s = serializer.serialize_struct("BlockIdEip1898", 1)?;
                s.serialize_field("blockHash", &format!("{:?}", x))?;
                s.end()
            }
            BlockId::Number(ref num) => num.serialize(serializer),
        }
    }
}

impl From<U64> for BlockId {
    fn from(source: U64) -> Self {
        BlockNumber::Number(source).into()
    }
}

impl From<BlockNumber> for BlockId {
    fn from(num: BlockNumber) -> Self {
        BlockId::Number(num)
    }
}

impl From<H256> for BlockId {
    fn from(hash: H256) -> Self {
        BlockId::Hash(hash)
    }
}
