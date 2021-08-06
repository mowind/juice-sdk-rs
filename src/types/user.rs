use crate::types::{Address, U64};
use serde::{Deserialize, Serialize};

/// The user type returned from contract calls.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct User {
    /// Address of the user
    #[serde(rename = "address")]
    pub addr: Address,
    /// Name of the user
    pub name: String,
    /// Mobile number of the user
    pub mobile: String,
    /// E-mail of the user
    pub email: String,
    /// Description
    pub desc: String,
    /// Status
    pub status: u8,
    /// Roles of the user
    pub roles: U64,
    /// Register time
    #[serde(rename = "registerTime")]
    pub register_time: u64,
    /// Update time
    #[serde(rename = "updateTime")]
    pub update_time: u64,
}

/// The user page info returned from contract calls.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UserPage {
    /// Total counts
    #[serde(rename = "totalCount")]
    pub total_count: u32,
    /// Total pages
    #[serde(rename = "totalPage")]
    pub total_page: u32,
    /// Page number
    #[serde(rename = "pageNum")]
    pub page_num: u32,
    /// Page size
    #[serde(rename = "page_size")]
    pub page_size: u32,
    /// User list
    pub items: Vec<User>,
}
