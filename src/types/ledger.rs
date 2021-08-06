use crate::types::{BlsPublic, Public};
use serde::{Deserialize, Serialize};

/// The ledger type returned from contract calls.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ledger {
    /// Name of the ledger
    #[serde(rename = "ledgerName")]
    pub name: String,
    /// ID of the ledger
    pub id: u32,
    /// Block timestamp
    pub timestamp: u64,
    /// Consensus nodes
    #[serde(rename = "consensusNodes")]
    pub consensus: Vec<CbftNode>,
    /// Observe nodes
    #[serde(rename = "observeNodes")]
    pub observe: Vec<CbftNode>,
    /// Status of the ledger
    #[serde(rename = "ledgerStatus")]
    pub status: u8,
}

/// The cbft node type returned from contract calls.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CbftNode {
    /// Node
    pub node: String,
    /// Bls public key
    #[serde(rename = "blsPubKey")]
    pub bls_pub_key: BlsPublic,
}

/// The ledger node type returned from contract calls.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LedgerNode {
    /// Public key of the ledger node
    #[serde(rename = "publicKey")]
    pub pub_key: Public,
    /// Node type
    #[serde(rename = "nodeType")]
    pub node_type: u8,
}
