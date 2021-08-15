use crate::backends::Installer;
use crate::parser::Package;

struct WindowsBackend;

impl Installer for WindowsBackend {
    fn install_package(package: &Package) {}
    fn install_package_list(packages: Vec<Package>) {}
}
