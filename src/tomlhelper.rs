use toml::Value;

const PACKAGES: &str = "packages.toml";

pub fn open_toml() -> Result<Value, String> {
    let file = std::fs::read_to_string(PACKAGES).expect(&format!("unable to read {}", PACKAGES));
    match file.parse::<Value>() {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("Error during parse: {}", e)),
    }
}
