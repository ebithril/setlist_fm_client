use http::StatusCode;
use std::fmt;

pub type Result<T> = std::result::Result<T, SetlistError>;

#[derive(Debug)]
pub struct SetlistError {
    pub status: StatusCode,
    pub message: String
}

impl SetlistError {
    pub fn new(status: StatusCode, message: String) -> Self {
        SetlistError {
            status,
            message
        }
    }
}

impl fmt::Display for SetlistError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StatusCode: {} Error: {}", self.status.as_str(), self.message)
    }
}
