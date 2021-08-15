use crate::backends::Installer;
use crate::packageconfig::Package;
use crate::systemconfig::Systemconfig;

struct WindowsBackend;

impl Installer for WindowsBackend {
    fn install_package(package: &Package, systemconfig: &Systemconfig) {}
    fn install_package_list(packages: Vec<Package>, systemconfig: &Systemconfig) {}
}
