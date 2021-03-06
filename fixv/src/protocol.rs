#![allow(dead_code)]
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FixMessage {
    pub msg_length: u64
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProtocolError {
    Incomplete,
    Malformed,
    MissingDelimiter,
    MissingVersion,
    MissingBodyLen,
    MissingChecksum,
    InvalidVersion,
    InvalidBodyLen,
    InvalidChecksum,
    IoError(String)
}

impl ProtocolError {
    fn as_str(&self) -> &'static str {
        match *self {
            ProtocolError::Incomplete => "FIX message incomplete",
            ProtocolError::Malformed => "Malformed message",
            ProtocolError::MissingDelimiter => "Missing delimiter",
            _ => "FIX protocol error",
        }
    }
//    fn from(err: std::io::Error) -> ProtocolError {
//        ProtocolError::IoError(err)
//    }
}

impl From<std::io::Error> for ProtocolError {
    fn from(err: std::io::Error) -> ProtocolError {
        ProtocolError::IoError(err.to_string())
    }
}

//impl Error for ProtocolError {
//    fn description(&self) -> &str {
//        &self.as_str()
//    }
//}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.as_str())
    }
}
