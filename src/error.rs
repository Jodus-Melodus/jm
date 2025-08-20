use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum ErrorType {
    Error,
    SyntaxError,
    NameError,
    TypeError,
}

pub struct Error {
    error_type: ErrorType,
    message: String,
    line: u128,
    column: u128,
}

impl Error {
    pub fn new(error_type: ErrorType, message: String, line: u128, column: u128) -> Self {
        Error {
            error_type,
            message,
            line,
            column,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}: {} in line {} column {}",
            self.error_type, self.message, self.line, self.column
        )
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error {{ error_type: {:?}, message: {}, line: {}, column: {} }}",
            self.error_type, self.message, self.line, self.column
        )
    }
}

impl Default for Error {
    fn default() -> Self {
        Error {
            error_type: ErrorType::Error,
            message: format!(""),
            line: 0,
            column: 0,
        }
    }
}
