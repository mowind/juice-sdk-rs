mod contract;
mod h768;
mod ledger;
mod node;
mod params;
mod user;

pub use ethereum_types::{
    BigEndianHash, Bloom as H2048, Public, H128, H160, H256, H512, H520, H64, U128, U256, U64,
};

// Address
pub type Address = H160;

// BlsPublic
pub type BlsPublic = H768;

pub use self::{
    contract::Contract,
    h768::H768,
    ledger::{CbftNode, Ledger, LedgerNode},
    node::Node,
    params::SysParams,
    user::User,
};

pub type LedgerNodeList = Vec<ledger::LedgerNode>;
