use std::num::ParseIntError;
use teloxide::RequestError;

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(msg: &str) -> Error {
        Error {
            message: msg.to_string(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<RequestError> for Error {
    fn from(error: RequestError) -> Error {
        Error::new(&error.to_string())
    }
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Error::new(&error.to_string())
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(error: mongodb::error::Error) -> Self {
        Error::new(&error.to_string())
    }
}

impl From<mongodb::bson::de::Error> for Error {
    fn from(error: mongodb::bson::de::Error) -> Self {
        Error::new(&error.to_string())
    }
}

impl From<mongodb::bson::ser::Error> for Error {
    fn from(error: mongodb::bson::ser::Error) -> Self {
        Error::new(&error.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::new(&error.to_string())
    }
}
impl From<mongodb::bson::oid::Error> for Error {
    fn from(error: mongodb::bson::oid::Error) -> Self {
        Error::new(&error.to_string())
    }
}

impl From<regex::Error> for Error {
    fn from(error: regex::Error) -> Self {
        Error::new(&error.to_string())
    }
}
