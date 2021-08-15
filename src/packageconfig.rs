pub enum Source {
    Git,
    Web, // Straight download, e.g. via curl
    Local,
}

pub enum Status {
    Ready,
    Succeeded,
    Failed,
}

pub struct Package {
    // todo
}

pub struct Target {
    name: String,
    host: Option<String>,
}

pub fn read_package_list(target: Target) -> Result<Vec<Package>, ()> {
    Err(())
}
