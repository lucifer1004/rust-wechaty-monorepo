use std::{error, fmt};

/// The errors that can occur during the endpoint resolution process.
pub enum EndpointError {
    InvalidToken,
    Network(reqwest::Error),
}

impl fmt::Debug for EndpointError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "EndpointError({})", self)
    }
}

impl fmt::Display for EndpointError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EndpointError::InvalidToken => write!(fmt, "Invalid token"),
            EndpointError::Network(err) => write!(fmt, "{}", err),
        }
    }
}

impl From<reqwest::Error> for EndpointError {
    fn from(err: reqwest::Error) -> EndpointError {
        EndpointError::Network(err)
    }
}

impl error::Error for EndpointError {}

#[derive(Clone, Copy)]
/// The errors that can occur during the rpc process.
pub enum RpcError {}
