use crate::types::U256;
use serde::de::{Deserializer, Error};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

/// Information about current blockchain syncing operations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncInfo {
    /// The block at which import began.
    pub starting_block: U256,

    /// The highest currently synced block.
    pub current_block: U256,

    /// The estimated highest block.
    pub highest_block: U256,
}

/// The current state of blockchain syncing operations.
#[derive(Debug, Clone, PartialEq)]
pub enum SyncState {
    /// Blockchain is syncing
    Syncing(SyncInfo),

    /// Blockchain is not syncing.
    NotSyncing,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum SyncStateVariants {
    Rpc(SyncInfo),
    Boolean(bool),
}

impl<'de> Deserialize<'de> for SyncState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: SyncStateVariants = Deserialize::deserialize(deserializer)?;
        match v {
            SyncStateVariants::Rpc(info) => Ok(SyncState::Syncing(info)),
            SyncStateVariants::Boolean(boolean) => {
                if !boolean {
                    Ok(SyncState::NotSyncing)
                } else {
                    Err(D::Error::custom("expected object or `false`, got `true`"))
                }
            }
        }
    }
}

impl Serialize for SyncState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            SyncState::Syncing(ref info) => info.serialize(serializer),
            SyncState::NotSyncing => false.serialize(serializer),
        }
    }
}
