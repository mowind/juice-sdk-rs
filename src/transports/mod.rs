pub mod http;

impl From<url::ParseError> for crate::Error {
    fn from(err: url::ParseError) -> Self {
        crate::Error::Transport(format!("failed to parse url: {}", err))
    }
}
