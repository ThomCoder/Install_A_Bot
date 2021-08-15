pub struct Systemconfig {
    name: Option<String>,
    install_cmd: String,
}

enum SupportedSystems {
    Linux,
    Mac,
    Windows,
}

pub fn read_system_config() -> Result<Systemconfig, ()> {
    Err(())
}
