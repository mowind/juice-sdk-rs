use crate::rpc::error::Error as RPCError;
use derive_more::{Display, From};
use serde_json::Error as SerdeError;
use std::io::Error as IoError;

pub type Result<T = ()> = std::result::Result<T, Error>;

/// Errors which can occur when attempting to generate resource uri.
#[derive(Debug, Display, From)]
pub enum Error {
    /// server is unreachable
    #[display(fmt = "Server is unreachable")]
    Unreachable,
    /// decoder error
    #[display(fmt = "Decoder error:{}", _0)]
    Decoder(String),
    /// invalid response
    #[display(fmt = "Got invalid response: {}", _0)]
    #[from(ignore)]
    InvalidResponse(String),
    /// transport error
    #[display(fmt = "Transport error: {}", _0)]
    #[from(ignore)]
    Transport(String),
    /// rpc error
    #[display(fmt = "RPC error: {}", _0)]
    Rpc(RPCError),
    /// io error
    #[display(fmt = "Recovery error: {}", _0)]
    Io(IoError),
    /// internal error
    #[display(fmt = "Internal error")]
    Internal,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use self::Error::*;
        match self {
            Unreachable | Decoder(_) | InvalidResponse(_) | Transport(_) | Internal => None,
            Rpc(ref e) => Some(e),
            Io(ref e) => Some(e),
        }
    }
}

impl From<SerdeError> for Error {
    fn from(err: SerdeError) -> Self {
        Error::Decoder(format!("{:?}", err))
    }
}

#[cfg(test)]
impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        use self::Error::*;
        match (self, other) {
            (Unreachable, Unreachable) | (Internal, Internal) => true,
            (Decoder(a), Decoder(b))
            | (InvalidResponse(a), InvalidResponse(b))
            | (Transport(a), Transport(b)) => a == b,
            (Rpc(a), Rpc(b)) => a == b,
            (Io(a), Io(b)) => a.kind() == b.kind(),
            _ => false,
        }
    }
}
