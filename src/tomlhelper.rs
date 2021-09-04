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

pub fn extract_toml_str_val(val: &Value) -> Option<String> {
    match val.as_str() {
        Some(v) => {
            if v.starts_with('\"') && v.ends_with('\"') {
                // if the value is wrapped in ", remove them
                Some(v[0..].to_owned())
            } else {
                Some(v.to_owned())
            }
        }
        None => None,
    }
}
