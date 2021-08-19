use toml::Value;

const PACKAGES: &str = "packages.toml";

pub fn open_toml() -> Result<Value, ()> {
    let file = std::fs::read_to_string(PACKAGES).expect("unable to read packages.toml");
    match file.parse::<Value>() {
        Ok(v) => Ok(v),
        Err(_) => Err(()),
    }
}
