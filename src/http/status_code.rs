#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusCode {
    OK = 200,
    Created = 201,
    Accepted = 202,
    NoContent = 204,
    MovedPermanently = 301,
    Found = 302,
    NotModified = 304,
    TemporaryRedirect = 307,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    InternalServerError = 500,
    NotImplemented = 501,
}

impl StatusCode {
    pub fn as_response(&self) -> String {
        format!("HTTP/1.1 {} {}\r\n", self.code(), self.reason_phrase())
    }

    pub fn reason_phrase(&self) -> &str {
        match self {
            StatusCode::OK => "OK",
            StatusCode::Created => "Created",
            StatusCode::Accepted => "Accepted",
            StatusCode::NoContent => "No Content",
            StatusCode::MovedPermanently => "Moved Permanently",
            StatusCode::Found => "Found",
            StatusCode::NotModified => "Not Modified",
            StatusCode::TemporaryRedirect => "Temporary Redirect",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::Unauthorized => "Unauthorized",
            StatusCode::Forbidden => "Forbidden",
            StatusCode::NotFound => "Not Found",
            StatusCode::InternalServerError => "Internal Server Error",
            StatusCode::NotImplemented => "Not Implemented",
        }
    }
    
    pub fn code(&self) -> u16 {
        *self as u16
    }
}