mod block;
mod bytes;
mod contract;
mod h768;
mod ledger;
mod log;
mod node;
mod params;
mod sync_state;
mod transaction;
mod transaction_id;
mod transaction_request;
mod user;

pub use ethereum_types::{
    BigEndianHash, Bloom as H2048, Public, H128, H160, H256, H512, H520, H64, U128, U256, U64,
};

/// Address
pub type Address = String;
/// Bls public key
pub type BlsPublic = H768;
/// Index in block
pub type Index = U64;

pub use self::{
    block::{Block, BlockHeader, BlockId, BlockNumber},
    bytes::Bytes,
    contract::Contract,
    h768::H768,
    ledger::{CbftNode, Ledger, LedgerNode},
    log::Log,
    node::Node,
    params::SysParams,
    sync_state::SyncState,
    transaction_id::TransactionId,
    transaction_request::{CallRequest, TransactionRequest},
    user::User,
};

pub use self::transaction::{RawTransaction, Receipt as TransactionReceipt, Transaction};

pub type LedgerNodeList = Vec<ledger::LedgerNode>;
