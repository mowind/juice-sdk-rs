use crate::types::{Address, Bytes, Index, H256, U256, U64};
use serde::{Deserialize, Serialize};

/// A log produced by a transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Log {
    /// Address
    pub address: Address,
    /// Topics
    pub topics: Vec<H256>,
    /// Data
    pub data: Bytes,
    /// Block hash
    #[serde(rename = "blockHash")]
    pub block_hash: Option<H256>,
    /// Block number
    #[serde(rename = "blockNumber")]
    pub block_number: Option<U64>,
    /// Transaction hash
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<H256>,
    /// Transaction index
    #[serde(rename = "transactionIndex")]
    pub transaction_index: Option<Index>,
    /// Log index in block
    #[serde(rename = "logIndex")]
    pub log_index: Option<U256>,
    /// Log index in transaction
    #[serde(rename = "transactionLogIndex")]
    pub transaction_log_index: Option<U256>,
    /// Removed
    pub removed: Option<bool>,
}

impl Log {
    /// Returns true if the log has removed.
    pub fn is_removed(&self) -> bool {
        match self.removed {
            Some(val_removed) => return val_removed,
            None => (),
        }
        false
    }
}
