use crate::types::{Address, Bytes, Index, Log, H2048, H256, U256, U64};
use serde::{Deserialize, Serialize};

/// Description of a Transaction, pending or in the chain.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Transaction {
    /// Hash
    pub hash: H256,
    /// Nonce
    pub nonce: Option<U64>,
    /// Block hash. None when pending.
    #[serde(rename = "blockHash")]
    pub block_hash: Option<H256>,
    /// Block number. None when pending.
    #[serde(rename = "blockNumber")]
    pub block_number: Option<U64>,
    /// Transaction index. None when pending.
    #[serde(rename = "transactionIndex")]
    pub transaction_index: Option<Index>,
    /// Sender
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<Address>,
    /// Recipient (None when contract creation)
    pub to: Option<Address>,
    /// Transfered value
    pub value: Option<U256>,
    /// Gas price
    #[serde(rename = "gasPrice")]
    pub gas_price: U256,
    /// Gas amount
    pub gas: U256,
    /// Input data
    pub input: Bytes,
    /// ECDSA recovery id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub v: Option<U64>,
    /// ECDSA signature r, 32 bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r: Option<U256>,
    /// ECDSA signature s, 32 bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s: Option<U256>,
    /// Raw transaction data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw: Option<Bytes>,
}

/// "Receipt" of an executed transaction: details of its execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Receipt {
    /// Transaction hash.
    pub transaction_hash: H256,
    /// Index within the block.
    #[serde(rename = "transactionIndex")]
    pub transaction_index: Index,
    /// Hash of the block this transaction was included within.
    #[serde(rename = "blockHash")]
    pub block_hash: Option<H256>,
    /// Number of the block this transaction was included within.
    #[serde(rename = "blockNumber")]
    pub block_number: Option<U64>,
    /// Cumulative gas used within the block after this was executed.
    #[serde(rename = "cumulativeGasUsed")]
    pub cumulative_gas_used: U256,
    /// Gas used by this transaction alone.
    #[serde(rename = "gasUsed")]
    pub gas_used: Option<U256>,
    /// Contract address created, or `None` if not deployment.
    #[serde(rename = "contractAddress")]
    pub contrace_address: Option<Address>,
    /// Logs generated within this transaction.
    pub logs: Vec<Log>,
    /// Status: either 1 (success) or 0 (failure).
    pub status: Option<U64>,
    /// State root
    pub root: Option<H256>,
    /// Logs bloom
    #[serde(rename = "logsBloom")]
    pub logs_bloom: H2048,
}

/// Raw bytes of a signed, but not yet sent transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawTransaction {
    /// Signed transaction as raw bytes
    pub raw: Bytes,
    /// Transaction details
    pub tx: RawTransactionDetails,
}

/// Details of a signed transaction.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RawTransactionDetails {
    /// Hash
    pub hash: H256,
    /// Nonce
    pub nonce: U256,
    /// Block hash. None when pending.
    #[serde(rename = "blockHash")]
    pub block_hash: Option<U256>,
    /// Block number. None when pending.
    #[serde(rename = "blockNumber")]
    pub block_number: Option<U64>,
    /// Transaction index. None when pending.
    #[serde(rename = "transactionIndex")]
    pub transation_index: Option<Index>,
    /// Sender
    pub from: Option<Address>,
    /// Recipient (None when contract creation)
    pub to: Option<Address>,
    /// Transfered value
    pub value: U256,
    /// Gas price
    #[serde(rename = "gasPrice")]
    pub gas_price: U256,
    /// gas amount
    pub gas: U256,
    /// Input data
    pub input: Bytes,
    /// ECDSA recovery id, set by Juice
    pub v: Option<U64>,
    /// ECDSA signature r, 32 bytes, set by Juice
    pub r: Option<U256>,
    /// ECDSA signature s, 32 bytes, set by Juice
    pub s: Option<U256>,
}
