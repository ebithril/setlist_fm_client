use std::fmt;
use url;

pub type Result<T> = std::result::Result<T, SetlistError>;

#[derive(Debug)]
pub enum SetlistError {
    Reqwest(reqwest::Error),
    ParseError(url::ParseError),
}

impl fmt::Display for SetlistError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Reqwest(ref e) => write!(f, "{:?}", e),
            Self::ParseError(ref e) => write!(f, "{:?}", e),
        }
    }
}

impl From<reqwest::Error> for SetlistError {
    fn from(err: reqwest::Error) -> SetlistError {
        Self::Reqwest(err)
    }
}

impl From<url::ParseError> for SetlistError {
    fn from(err: url::ParseError) -> SetlistError {
        Self::ParseError(err)
    }
}
