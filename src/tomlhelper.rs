use toml::Value;

use crate::errors::{Error, ErrorCode};

const PACKAGES: &str = "packages.toml";

pub fn open_toml() -> Result<Value, Error> {
    let file = std::fs::read_to_string(PACKAGES).expect(&format!("unable to read {}", PACKAGES));
    match file.parse::<Value>() {
        Ok(v) => Ok(v),
        Err(e) => Err(Error::with_msg(ErrorCode::ParseError, format!("{}", e))),
    }
}
