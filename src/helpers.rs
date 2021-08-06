use crate::{error, rpc, Error};
use serde::de::DeserializeOwned;

/// Take any type which is deserializable from rpc::Value and such a value and
/// yields the deserialized value.
pub fn decode<T: serde::de::DeserializeOwned>(value: rpc::Value) -> error::Result<T> {
    serde_json::from_value(value).map_err(Into::into)
}

/// Serialize a type. Panics if the type is reuturns error during serialization.
pub fn serialize<T: serde::Serialize>(t: &T) -> rpc::Value {
    serde_json::to_value(t).expect("Types never fail to serialize.")
}

/// Serializes a request to string. Panics if the type returns error during serialization.
pub fn to_string<T: serde::Serialize>(request: &T) -> String {
    serde_json::to_string(&request).expect("String serialization never fails.")
}

/// Build a JSON-RPC request.
pub fn build_request(id: usize, method: &str, params: Vec<rpc::Value>) -> rpc::Call {
    rpc::Call::MethodCall(rpc::MethodCall {
        jsonrpc: Some(rpc::Version::V2),
        method: method.into(),
        params: rpc::Params::Array(params),
        id: rpc::Id::Num(id as u64),
    })
}

/// Parse bytes slice into JSON-RPC response.
pub fn to_response_from_slice(response: &[u8]) -> error::Result<rpc::Response> {
    arbitrary_precision_deserialize_workaround(response)
        .map_err(|e| Error::InvalidResponse(format!("{:?}", e)))
}

/// Deserialize bytes into T.
pub fn arbitrary_precision_deserialize_workaround<T>(bytes: &[u8]) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    if cfg!(feature = "arbitrary_precision") {
        serde_json::from_value(serde_json::from_slice(bytes)?)
    } else {
        serde_json::from_slice(bytes)
    }
}

/// Parse a Vec of `rpc::Output` into `Result`.
pub fn to_results_from_outputs(
    outputs: Vec<rpc::Output>,
) -> error::Result<Vec<error::Result<rpc::Value>>> {
    Ok(outputs.into_iter().map(to_result_from_output).collect())
}

/// Parse `rpc::Output` into `Result`.
pub fn to_result_from_output(output: rpc::Output) -> error::Result<rpc::Value> {
    match output {
        rpc::Output::Success(success) => Ok(success.result),
        rpc::Output::Failure(failure) => Err(error::Error::Rpc(failure.error)),
    }
}
