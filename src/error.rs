use std::fmt;

pub type Result<T> = std::result::Result<T, SetlistError>;

#[derive(Debug)]
pub enum SetlistError {
    Reqwest(reqwest::Error),
}

impl fmt::Display for SetlistError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Reqwest(ref e) =>
                write!(f, "{:?}", e)
        }
    }
}

impl From<reqwest::Error> for SetlistError {
    fn from(err: reqwest::Error) -> SetlistError {
        Self::Reqwest(err)
    }
}
