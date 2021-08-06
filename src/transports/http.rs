use crate::{
    error::{Error, Result},
    helpers, RequestId, Transport,
};
use futures::future::BoxFuture;
use jsonrpc_core::types::{Call, Output, Request, Value};
use reqwest::{Client, Url};
use serde::de::DeserializeOwned;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

/// HTTP Transport
#[derive(Debug, Clone)]
pub struct Http {
    client: Client,
    inner: Arc<Inner>,
}

#[derive(Debug)]
struct Inner {
    url: Url,
    id: AtomicUsize,
}

impl Http {
    /// Create new HTTP transport connecting to given URL.
    pub fn new(url: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut builder = Client::builder();
        let client = builder
            .build()
            .map_err(|err| Error::Transport(format!("failed to build client: {}", err)))?;
        Ok(Self::with_client(client, url.parse()?))
    }

    /// Like `new` but with a user provided client instance.
    pub fn with_client(client: Client, url: Url) -> Self {
        Self {
            client,
            inner: Arc::new(Inner {
                url,
                id: AtomicUsize::new(0),
            }),
        }
    }

    fn next_id(&self) -> RequestId {
        self.inner.id.fetch_add(1, Ordering::AcqRel)
    }

    fn new_request(&self) -> (Client, Url) {
        (self.client.clone(), self.inner.url.clone())
    }
}

async fn execute_rpc<T: DeserializeOwned>(
    client: &Client,
    url: Url,
    request: &Request,
    id: RequestId,
) -> Result<T> {
    log::debug!(
        "[id:{}] sending request: {:?}",
        id,
        serde_json::to_string(&request)?
    );
    let response = client
        .post(url)
        .json(request)
        .send()
        .await
        .map_err(|err| Error::Transport(format!("failed to send request: {}", err)))?;
    let status = response.status();
    let response = response
        .bytes()
        .await
        .map_err(|err| Error::Transport(format!("failed to read response bytes: {}", err)))?;
    log::debug!(
        "[id:{}] received response: {:?}",
        id,
        String::from_utf8_lossy(&response).as_ref()
    );
    if !status.is_success() {
        return Err(Error::Transport(format!(
            "response status code is not success: {}",
            status,
        )));
    }
    helpers::arbitrary_precision_deserialize_workaround(&response)
        .map_err(|err| Error::Transport(format!("failed to deserialize response: {}", err)))
}

impl Transport for Http {
    type Out = BoxFuture<'static, Result<Value>>;

    fn prepare(&self, method: &str, params: Vec<Value>) -> (RequestId, Call) {
        let id = self.next_id();
        let request = helpers::build_request(id, method, params);
        (id, request)
    }

    fn send(&self, id: RequestId, call: Call) -> Self::Out {
        let (client, url) = self.new_request();
        Box::pin(async move {
            let output: Output = execute_rpc(&client, url, &Request::Single(call), id).await?;
            helpers::to_result_from_output(output)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn server(
        req: hyper::Request<hyper::Body>,
    ) -> hyper::Result<hyper::Response<hyper::Body>> {
        use hyper::body::HttpBody;

        let expected = r#"{"jsonrpc":"2.0","method":"eth_getAccounts","params":[],"id":0}"#;
        let response = r#"{"jsonrpc":"2.0","id":0,"result":"x"}"#;

        assert_eq!(req.method(), &hyper::Method::POST);
        assert_eq!(req.uri().path(), "/");
        let mut content: Vec<u8> = vec![];
        let mut body = req.into_body();
        while let Some(Ok(chunk)) = body.data().await {
            content.extend(&*chunk);
        }
        assert_eq!(std::str::from_utf8(&*content), Ok(expected));

        Ok(hyper::Response::new(response.into()))
    }

    #[tokio::test]
    async fn should_make_a_request() {
        use hyper::service::{make_service_fn, service_fn};

        // given
        let addr = "127.0.0.1:3001";
        // start server
        let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(server)) });
        let server = hyper::Server::bind(&addr.parse().unwrap()).serve(service);
        tokio::spawn(async move {
            println!("Listening on http://{}", addr);
            server.await.unwrap();
        });

        // when
        let client = Http::new(&format!("http://{}", addr)).unwrap();
        println!("Sending request");
        let response = client.execute("eth_getAccounts", vec![]).await;
        println!("Got response");

        // then
        assert_eq!(response, Ok(Value::String("x".into())));
    }
}
