use crate::backends::Installer;
use crate::packageconfig::Package;
use crate::systemconfig::Systemconfig;

struct MacBackend;

impl Installer for MacBackend {
	fn install_package(package: &mut Package, systemconfig: &Systemconfig) -> Result<(), ()> {
		Ok(())
	}

	fn install_package_list(packages: &mut Vec<Package>, systemconfig: &Systemconfig) -> Result<(), ()> {
		Ok(())
	}
}