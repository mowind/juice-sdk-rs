use serde::{Deserialize, Serialize};

/// The system parameters type returned from contract calls.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SysParams {
    /// Block gas limit
    #[serde(rename = "BlockGasLimit")]
    pub block_gas_limit: u64,
    /// Transaction gas limit
    #[serde(rename = "TxGasLimit")]
    pub tx_gas_limit: u64,
    /// Is transaction use gas
    #[serde(rename = "IsTxUseGas")]
    pub is_tx_use_gas: bool,
    /// Is produce empty block
    #[serde(rename = "IsProduceEmptyBlock")]
    pub is_produce_empty_block: bool,
    /// Enable deploy
    #[serde(rename = "EnableDeploy")]
    pub enable_deploy: bool,
}
