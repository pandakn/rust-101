use crate::http::{Error, Method, QueryString, Result};
use std::convert::TryFrom;
use std::str;

#[derive(Debug)]
pub struct Request {
    method: Method,
    path: String,
    query: Option<QueryString>,
}

impl Request {
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query.as_ref()
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = Error;

    fn try_from(buf: &[u8]) -> Result<Self> {
        let req = str::from_utf8(&buf)?;
        println!("{}", req);

        // GET /hello?name=foo HTTP/1.1
        let mut req = req.split_whitespace();
        let method = req.next().ok_or(Error::InvalidMethod)?;
        let mut path = req.next().ok_or(Error::InvalidRequest)?;
        let protocol = req.next().ok_or(Error::InvalidRequest)?;
        if protocol != "HTTP/1.1" {
            return Err(Error::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        let query_string = match path.find("?") {
            Some(i) => {
                let qs: &str = &path[i + 1..];
                path = &path[..i];
                Some(QueryString::from(qs))
            }
            None => None,
        };

        Ok(Self {
            method,
            path: path.to_string(),
            query: query_string,
        })
    }
}
