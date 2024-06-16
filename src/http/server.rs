use crate::http::{HttpStatus, Method, Request, Response, Result};
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) -> Result<()> {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(self.addr)?; // `?` -> try  operator

        for stream in listener.incoming() {
            let mut stream = stream?;
            let mut buf: [u8; 1024] = [0; 1024];
            stream.read(&mut buf)?;

            let req = Request::try_from(&buf[..])?;
            println!("{:#?}", req);

            let resp = match req.method() {
                Method::GET => match req.path().as_str() {
                    "/" => Response::new(HttpStatus::OK, Some("Hello World!".to_string())),
                    "/hey" => Response::new(HttpStatus::OK, Some("Sub y'all".to_string())),
                    _ => Response::new(HttpStatus::BadRequest, None),
                },
                _ => Response::new(HttpStatus::BadRequest, None),
            };

            resp.send(&mut stream)?;
        }

        Ok(())
    }
}
