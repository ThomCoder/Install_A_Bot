use std::fmt::Display;

use crate::tomlhelper;
use toml::Value;

const PACKAGES_IDX: &str = "packages";
const PACKAGECONFIG_IDX: &str = "packagedefinition";
const SOURCE_TYPE_IDX: &str = "source";
const SOURCE_URL_IDX: &str = "url";
const CMDS_IDX: &str = "install_cmds";

#[derive(Debug)]
pub enum Source {
    Git,
    Web, // Straight download, e.g. via curl
    Local,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Status {
    ParseErr,
    Ready,
    Succeeded,
    Failed,
}

pub struct PackageConfig {
    toml: Value,
    packageconfigs: Option<Value>,
}

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub status: Status,
    pub source: Option<(Source, String)>,
    pub cmd: Option<Vec<String>>,
}

pub struct Target {
    pub name: String,
    pub host: Option<String>,
}

impl PackageConfig {
    pub fn new() -> Result<PackageConfig, ()> {
        match tomlhelper::open_toml() {
            Ok(t) => {
                let packagesconfig = t.get(PACKAGECONFIG_IDX).map(|v| v.clone());
                Ok(PackageConfig {
                    toml: t,
                    packageconfigs: packagesconfig,
                })
            }
            Err(_) => Err(()),
        }
    }

    pub fn read_package_list(&self, target: Target) -> Vec<Package> {
        let mut packages: Vec<Package> = Vec::new();
        let packages_raw = self.read_packages(&target);
        for packname in &packages_raw {
            let pack = self.make_package(packname);
            packages.push(pack);
        }
        packages
    }

    fn read_packages(&self, target: &Target) -> Vec<String> {
        let mut packages: Vec<String> = Vec::new();
        if let Some(main) = self.toml.get(&target.name) {
            packages = self.get_packages_from_target(&main);
            // try to get host specific packages if we have a hostname specified
            if let Some(host) = &target.host {
                if let Some(sub) = main.get(&host) {
                    let mut sub_packages = self.get_packages_from_target(sub);
                    packages.append(&mut sub_packages);
                }
            }
        }
        packages
    }

    fn get_packages_from_target(&self, root: &Value) -> Vec<String> {
        let mut packages: Vec<String> = Vec::new();
        if let Some(pack_toml) = root.get(PACKAGES_IDX) {
            if pack_toml.is_array() {
                for pack in pack_toml.as_array().unwrap() {
                    if let Some(packstr) = pack.as_str() {
                        packages.push(packstr.to_string());
                    } else {
                        println!("invalid package name: {}", pack);
                    }
                }
            }
        }
        packages
    }

    fn make_package(&self, package: &str) -> Package {
        if let Some(packageconfigs) = &self.packageconfigs {
            if let Some(pack) = packageconfigs.get(package) {
                let mut parse_ok = true;
                let mut source: Option<(Source, String)> = None;
                let source_type = pack.get(SOURCE_TYPE_IDX);
                let source_url = pack.get(SOURCE_URL_IDX);
                if source_type.is_some() && source_url.is_none()
                    || source_type.is_none() && source_url.is_some()
                {
                    println!(
                        "invalid package definition: source without url or url without sourcetype"
                    ); // todo?
                    parse_ok = false;
                }
                if source_type.is_some() && source_url.is_some() {
                    if let Some(stype) = source_type.unwrap().as_str() {
                        let stype_internal = Source::from_value(stype);
                        let surl = source_url.unwrap().as_str();
                        if stype_internal.is_ok() && surl.is_some() {
                            source = Some((stype_internal.unwrap(), surl.unwrap().to_string()));
                        }
                    }
                    if source.is_none() {
                        parse_ok = false;
                        println!("err reading source cfg");
                    }
                }
                let mut cmds: Option<Vec<String>> = None;
                if let Some(cmdsval) = pack.get(CMDS_IDX) {
                    if let Some(cmdsarr) = cmdsval.as_array() {
                        let mut cmdsvec: Vec<String> = Vec::new();
                        for cmdval in cmdsarr {
                            let cmd = cmdval.as_str();
                            if cmd.is_some() {
                                cmdsvec.push(cmd.unwrap().to_string());
                            } else {
                                parse_ok = false;
                                println!("failed to read packagecmd: {}", cmdval);
                            }
                        }
                        cmds = Some(cmdsvec);
                    }
                }
                let status = match parse_ok {
                    true => Status::Ready,
                    false => Status::ParseErr,
                };
                return Package {
                    name: package.to_string(),
                    status: status,
                    source: source,
                    cmd: cmds,
                };
            } // detailed package
        }
        return Package {
            name: package.to_string(),
            status: Status::Ready,
            source: None,
            cmd: None,
        }; // normal package without details
    }
}

impl Source {
    fn from_value(value: &str) -> Result<Source, ()> {
        match value.to_lowercase().as_str() {
            "git" => Ok(Source::Git),
            "web" => Ok(Source::Web),
            "local" => Ok(Source::Local),
            _ => Err(()),
        }
    }

    fn get_value(&self) -> &str {
        match self {
            Source::Git => "Git",
            Source::Web => "Web",
            Source::Local => "Local",
        }
    }
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_value())
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Status::Ready => "Status: Ready",
            Status::ParseErr => "Status: Parse Error",
            Status::Succeeded => "Status: Success",
            Status::Failed => "Status: Failure",
        };
        write!(f, "{}", value)
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Package '{}' [{}]", self.name, self.status)?;
        let mut details: String = String::new();
        if let Some(source) = &self.source {
            details.push_str(&format!("Source: [Type: {}, Url: {}]", source.0, source.1));
        }
        if let Some(cmds) = &self.cmd {
            if details.len() > 0 {
                details.push_str(", ");
            }
            details.push_str("Cmds: [");
            for (i, cmd) in cmds.iter().enumerate() {
                details.push_str(&format!("'{}'", cmd));
                if i + 1 < cmds.len() {
                    details.push(',')
                }
            }
            details.push_str("]")
        }
        if details.len() > 0 {
            write!(f, "\n -> {}", details)?;
        }
        Ok(())
    }
}
