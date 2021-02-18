use std::{error, fmt};

use wechaty_puppet::PuppetError;

pub enum WechatyError {
    Puppet(PuppetError),
}

impl fmt::Debug for WechatyError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "WechatyError({})", self)
    }
}

impl fmt::Display for WechatyError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WechatyError::Puppet(e) => write!(fmt, "Puppet error: {}", e),
        }
    }
}

impl From<PuppetError> for WechatyError {
    fn from(e: PuppetError) -> Self {
        WechatyError::Puppet(e)
    }
}

impl error::Error for WechatyError {}
