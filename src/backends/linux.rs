use crate::backends::Installer;
use crate::packageconfig::Package;
use crate::systemconfig::Systemconfig;

struct LinuxBackend;

impl Installer for LinuxBackend {
	fn install_package(package: &mut Package, systemconfig: &Systemconfig) -> Result<(), ()> {
		Ok(())
	}

	fn install_package_list(packages: &mut Vec<Package>, systemconfig: &Systemconfig) -> Result<(), ()> {
		Ok(())
	}
}