use crate::tomlhelper;

pub struct Systemconfig {
    pub name: Option<String>,
    pub install_cmd: String,
}

pub enum SupportedSystems {
    Linux,
    Mac,
    Windows,
}

impl SupportedSystems {
    pub fn value(&self) -> &str {
        match self {
            SupportedSystems::Linux => "linux",
            SupportedSystems::Mac => "mac",
            SupportedSystems::Windows => "windows",
        }
    }

    pub fn from_value(value: &str) -> Option<SupportedSystems> {
        match value {
            "linux" => Some(SupportedSystems::Linux),
            "mac" => Some(SupportedSystems::Mac),
            "windows" => Some(SupportedSystems::Windows),
            _ => None,
        }
    }

    fn check_platform(platform: &str) -> bool {
        let system = SupportedSystems::from_value(platform);
        system.is_some()
    }
}

pub fn read_system_config(
    platform: Option<&str>,
    distribution: Option<&str>,
) -> Result<Systemconfig, String> {
    if let Some(platform) = &platform {
        if !SupportedSystems::check_platform(&platform) {
            return Err(format!("Platform {} not supported", platform));
        }
    }

    let toml = tomlhelper::open_toml()?;
    let systemconfig = &toml["systemconfig"];
    let mut install_cmd_toml: Option<String> = None;

    if let Some(val) = systemconfig.get("install_cmd") {
        install_cmd_toml = Some(val.to_string());
    }
    if let Some(target) = &platform {
        if let Some(specific_config) = systemconfig.get(target) {
            if let Some(val) = specific_config.get("install_cmd") {
                install_cmd_toml = Some(val.to_string());
            }
            if let Some(distro) = &distribution {
                if let Some(distro_config) = specific_config.get(distro) {
                    if let Some(val) = distro_config.get("install_cmd") {
                        install_cmd_toml = Some(val.to_string());
                    }
                }
            }
        }
    }

    if install_cmd_toml.is_none() {
        return Err("Missing install command".to_string());
    }

    let sysname = match distribution {
        Some(distro) => format!("{}.{}", platform.unwrap_or(""), distro),
        None => platform.unwrap_or("").to_string(),
    };
    let mut name: Option<String> = None;
    if sysname.len() > 0 {
        name = Some(sysname)
    }

    Ok(Systemconfig {
        name: name,
        install_cmd: install_cmd_toml.unwrap(),
    })
}
