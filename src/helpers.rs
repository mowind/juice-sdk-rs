use crate::{error, rpc, Error};
use futures::{
    task::{Context, Poll},
    Future,
};
use pin_project::pin_project;
use serde::de::DeserializeOwned;
use std::{marker::PhantomData, pin::Pin};

/// Take any type which is deserializable from rpc::Value and such a value and
/// yields the deserialized value.
pub fn decode<T: serde::de::DeserializeOwned>(value: rpc::Value) -> error::Result<T> {
    serde_json::from_value(value).map_err(Into::into)
}

/// Calls decode on the result of the wrapped future.
#[pin_project]
#[derive(Debug)]
pub struct CallFuture<T, F> {
    #[pin]
    inner: F,
    _maker: PhantomData<T>,
}

impl<T, F> CallFuture<T, F> {
    /// Create a new CallFuture wrapping the inner future.
    pub fn new(inner: F) -> Self {
        CallFuture {
            inner,
            _maker: PhantomData,
        }
    }
}

impl<T, F> Future for CallFuture<T, F>
where
    T: serde::de::DeserializeOwned,
    F: Future<Output = error::Result<rpc::Value>>,
{
    type Output = error::Result<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let this = self.project();
        let x = ready!(this.inner.poll(cx));
        Poll::Ready(x.and_then(decode))
    }
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

#[macro_use]
#[cfg(test)]
pub mod tests {
    use crate::error::{self, Error};
    use crate::rpc;
    use crate::{RequestId, Transport};
    use futures::future;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::marker::Unpin;
    use std::rc::Rc;

    type Result<T> = Box<dyn futures::Future<Output = error::Result<T>> + Send + Unpin>;

    #[derive(Debug, Default, Clone)]
    pub struct TestTransport {
        asserted: usize,
        requests: Rc<RefCell<Vec<(String, Vec<rpc::Value>)>>>,
        responses: Rc<RefCell<VecDeque<rpc::Value>>>,
    }

    impl Transport for TestTransport {
        type Out = Result<rpc::Value>;

        fn prepare(&self, method: &str, params: Vec<rpc::Value>) -> (RequestId, rpc::Call) {
            let request = super::build_request(1, method, params.clone());
            self.requests.borrow_mut().push((method.into(), params));
            (self.requests.borrow_mut().len(), request)
        }

        fn send(&self, id: RequestId, request: rpc::Call) -> Result<rpc::Value> {
            Box::new(future::ready(
                match self.responses.borrow_mut().pop_front() {
                    Some(response) => Ok(response),
                    None => {
                        println!("Unexpected request (id: {:?}): {:?}", id, request);
                        Err(Error::Unreachable)
                    }
                },
            ))
        }
    }

    impl TestTransport {
        pub fn set_response(&mut self, value: rpc::Value) {
            *self.responses.borrow_mut() = vec![value].into();
        }

        pub fn add_response(&mut self, value: rpc::Value) {
            self.responses.borrow_mut().push_back(value);
        }

        pub fn assert_request(&mut self, method: &str, params: &[String]) {
            let idx = self.asserted;
            self.asserted += 1;

            let (m, p) = self
                .requests
                .borrow()
                .get(idx)
                .expect("Expected result.")
                .clone();
            assert_eq!(&m, method);
            let p: Vec<String> = p
                .into_iter()
                .map(|p| serde_json::to_string(&p).unwrap())
                .collect();
            assert_eq!(p, params);
        }

        pub fn assert_no_more_requests(&self) {
            let requests = self.requests.borrow();
            assert_eq!(
                self.asserted,
                requests.len(),
                "Expected no more requests, got: {:?}",
                &requests[self.asserted..]
            );
        }
    }

    macro_rules! rpc_test {
        // With parameters (implicit test name)
        (
            $client: ident : $name: ident : $test_name: ident $(, $param: expr) + => $method: expr, $results: expr;
            $returned: expr => $expected: expr
        ) => {
	        #[test]
            fn $test_name() {
                // given
                let mut transport = $crate::helpers::tests::TestTransport::default();
                transport.set_response($returned);
                let result = {
                    let juice = $client::new(&transport, true);

                    // when
                    juice.$name($($param.into(), )+)
                };

                // then
                transport.assert_request($method, &$results.into_iter().map(Into::into).collect::<Vec<_>>());
                transport.assert_no_more_requests();
                let result = futures::executor::block_on(result);
                assert_eq!(result, Ok($expected.into()));
            }
        };
        // With parameters (implicit test name)
        (
            $client: ident : $name: ident $(, $param: expr)+ => $method: expr, $results: expr;
            $returned: expr => $expected: expr
        ) => {
            rpc_test!(
                $client : $name : $name $(, $param)+ => $method, $results;
                $returned => $expected
            );
        };

        // No params entry point (explicit name)
        (
            $client: ident: $name: ident: $test_name: ident => $method: expr;
            $returned: expr => $expected: expr
        ) => {
            #[test]
            fn $test_name() {
                // given
                let mut transport = $crate::helpers::tests::TestTransport::default();
                transport.set_response($returned);
                let result = {
                    let juice = $client::new(&transport, true);

                    // when
                    juice.$name()
                };

                // then
                transport.assert_request($method, &[]);
                transport.assert_no_more_requests();
                let result = futures::executor::block_on(result);
                assert_eq!(result, Ok($expected.into()));
            }
            };

            // No params entry point
            (
                $client: ident : $name: ident => $method: expr;
                $returned: expr => $expected: expr
            ) => {
                rpc_test!(
                    $client: $name : $name => $method;
                    $returned => $expected
                );
            }
        }
}
