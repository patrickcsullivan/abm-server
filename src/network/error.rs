use serde_json;
use std::{convert::From, fmt};
use tungstenite;

pub type NetworkResult<T> = Result<T, NetworkError>;

#[derive(Debug)]
pub enum NetworkError {
    Serde(serde_json::Error),
    Tungstenite(tungstenite::error::Error),
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Use the underlying implementations of `Display`.
            NetworkError::Serde(ref err) => write!(f, "Serde error: {}", err),
            NetworkError::Tungstenite(ref err) => write!(f, "Tungstenite error: {}", err),
        }
    }
}

impl From<serde_json::Error> for NetworkError {
    fn from(err: serde_json::Error) -> NetworkError {
        NetworkError::Serde(err)
    }
}

impl From<tungstenite::error::Error> for NetworkError {
    fn from(err: tungstenite::error::Error) -> NetworkError {
        NetworkError::Tungstenite(err)
    }
}
