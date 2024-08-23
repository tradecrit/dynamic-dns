#[derive(Debug)]
pub struct AppError {
    pub message: String,
}

impl AppError {
    pub fn new(message: String) -> AppError {
        AppError {
            message
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppError {
    fn description(&self) -> &str {
        &self.message
    }
}
