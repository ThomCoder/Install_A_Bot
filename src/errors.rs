use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum ErrorCode {
    InvalidParameter,
    InvalidConfig,
    ParseError,
    HostError, // Signals error while interacting with host system
    NetworkError,
}

#[derive(Debug)]
pub struct Error {
    code: ErrorCode,
    msg: Option<String>,
}

impl Error {
    pub fn new(code: ErrorCode) -> Error {
        Error {
            code: code,
            msg: None,
        }
    }

    pub fn with_msg(code: ErrorCode, msg: String) -> Error {
        Error {
            code: code,
            msg: Some(msg),
        }
    }

    pub fn code(&self) -> &ErrorCode {
        &self.code
    }

    pub fn msg(&self) -> Option<&String> {
        self.msg.as_ref()
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            ErrorCode::InvalidParameter => "Invalid Parameter",
            ErrorCode::ParseError => "Parse Error",
            ErrorCode::InvalidConfig => "Invalid Configuration",
            ErrorCode::HostError => "Interaction with host system failed",
            ErrorCode::NetworkError => "Network request failed"
        };
        write!(f, "{}", code)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            &self.code,
            self.msg.as_ref().unwrap_or(&String::new())
        )
    }
}
