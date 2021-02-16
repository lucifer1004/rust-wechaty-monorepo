use std::{error, fmt};

/// The errors that can occur during the communication with the puppet.
pub enum PuppetError {
    Network,
    Unsupported(String),
    UnknownPayloadType,
    UnknownMessageType,
}

impl fmt::Debug for PuppetError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "PuppetError({})", self)
    }
}

impl fmt::Display for PuppetError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PuppetError::Network => write!(fmt, "Network failure"),
            PuppetError::Unsupported(function) => write!(fmt, "Unsupported function: {}", function),
            PuppetError::UnknownPayloadType => write!(fmt, "Unknown payload type"),
            PuppetError::UnknownMessageType => write!(fmt, "Unknown message type"),
        }
    }
}

// impl From<reqwest::Error> for PuppetError {
//     fn from(err: reqwest::Error) -> PuppetError {
//         PuppetError::Network(err)
//     }
// }

impl error::Error for PuppetError {}
