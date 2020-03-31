use serde_json;
use std::{convert::From, fmt};
use tungstenite;

pub type ClientResult<T> = Result<T, ClientError>;

#[derive(Debug)]
pub enum ClientError {
    Serde(serde_json::Error),
    Tungstenite(tungstenite::error::Error),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Use the underlying implementations of `Display`.
            ClientError::Serde(ref err) => write!(f, "Serde error: {}", err),
            ClientError::Tungstenite(ref err) => write!(f, "Tungstenite error: {}", err),
        }
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(err: serde_json::Error) -> ClientError {
        ClientError::Serde(err)
    }
}

impl From<tungstenite::error::Error> for ClientError {
    fn from(err: tungstenite::error::Error) -> ClientError {
        ClientError::Tungstenite(err)
    }
}
