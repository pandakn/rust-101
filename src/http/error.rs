use std::convert::From;
use std::fmt::Display;

pub enum Error {
    InvalidRequest,
    InvalidProtocol,
    InvalidMethod,
    IO(String),
    Utf8(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
            Self::IO(e) => e,
            Self::Utf8(e) => e,
        };

        write!(f, "Error: {}", msg)
    }
}

//
/***
 * TODO : convert `io::Error` to my custom `Error` enum
 * * must convert cuz I use `?` operator, and I wanna handle any `io::Error` myself
***/
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e.to_string())
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::Utf8(e.to_string())
    }
}
