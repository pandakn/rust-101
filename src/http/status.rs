use std::fmt::Display;

pub enum HttpStatus {
    OK = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::OK => "200 OK",
            Self::BadRequest => "400 Bad Request",
            Self::NotFound => "404 Not Found",
            Self::InternalServerError => "500 Internal Server Error",
        };

        write!(f, "{}", msg)
    }
}
