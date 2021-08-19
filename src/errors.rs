use std::fmt::Display;

pub enum ErrorCode {
    InvalidParameter,
    InvalidConfig,
    ParseError,
}

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
