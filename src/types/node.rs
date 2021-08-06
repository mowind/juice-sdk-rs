use crate::types::{Address, BlsPublic, Public};
use serde::{Deserialize, Serialize};

/// The node type returned from contract calls.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Node {
    /// Name of the node
    pub name: String,
    /// Owner of the node
    pub owner: Address,
    /// Description
    pub desc: String,
    /// Node type
    #[serde(rename = "nodeType")]
    pub node_type: u8,
    /// Public key
    #[serde(rename = "publicKey")]
    pub public_key: Public,
    /// Bls public key
    #[serde(rename = "blsPubKey")]
    pub bls_pub_key: BlsPublic,
    /// Host IP address
    #[serde(rename = "hostAddress")]
    pub host_addr: String,
    /// RPC port
    #[serde(rename = "rpcPort")]
    pub rpc_port: u32,
    /// P2P port
    #[serde(rename = "p2pPort")]
    pub p2p_port: u32,
    /// Node status
    pub status: u8,
    /// Is root?
    pub root: bool,
    /// Create time
    #[serde(rename = "createTime")]
    pub create_time: u64,
    /// Update time
    #[serde(rename = "updateTime")]
    pub update_time: u64,
}
