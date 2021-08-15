mod backends;
mod install;
mod packageconfig;
mod systemconfig;

fn main() {
    let sys = systemconfig::read_system_config(
        "packages.toml",
        Some("linux".to_string()),
        Some("arch".to_string()),
    )
    .unwrap();
    println!(
        "Name: {} | install_cmd: {}",
        sys.name.unwrap_or(String::new()),
        sys.install_cmd
    )
}
