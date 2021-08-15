use crate::backends::Installer;
use crate::packageconfig::Package;

struct MacBackend;

impl Installer for MacBackend {
	fn install_package(package: &Package) {}
	fn install_package_list(packages: Vec<Package>) {}
}