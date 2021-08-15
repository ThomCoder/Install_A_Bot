use crate::backends::Installer;
use crate::packageconfig::Package;
use crate::systemconfig::Systemconfig;

struct MacBackend;

impl Installer for MacBackend {
	fn install_package(package: &Package, systemconfig: &Systemconfig) {}
	fn install_package_list(packages: Vec<Package>, systemconfig: &Systemconfig) {}
}