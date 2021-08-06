use crate::types::Address;
use serde::{Deserialize, Serialize};

/// The contract type returned from contract calls.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Contract {
    /// Name of the contract
    pub name: String,
    /// Version of the contract
    pub version: String,
    /// Address of the contract
    #[serde(rename = "address")]
    pub addr: Address,
    /// Owner of the contract
    pub owner: Address,
}
