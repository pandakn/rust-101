use crate::http::{Error, Result};
use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    PUT,
    POST,
    DELETE,
}

impl FromStr for Method {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            _ => Err(Error::InvalidMethod),
        }
    }
}
