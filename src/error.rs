use teloxide::RequestError;

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
}

impl RuntimeError {
    pub fn new(msg: &str) -> RuntimeError {
        RuntimeError {
            message: msg.to_string(),
        }
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for RuntimeError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<RequestError> for RuntimeError {
    fn from(error: RequestError) -> RuntimeError {
        RuntimeError::new(&error.to_string())
    }
}
