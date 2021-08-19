use crate::backends::Installer;
use crate::packageconfig::Package;
use crate::systemconfig::Systemconfig;

struct LinuxBackend;

impl Installer for LinuxBackend {
	fn install_package(package: &mut Package, systemconfig: &Systemconfig) -> Result<(), ()> {
		match package.source.as_ref() {
			Some(src) => {

				match src {
					Git => {

					}

					Web => {

					}

					Local => {

					}
				}

			}

			None => {
				return Err(())
			}
		}

		Ok(())
	}

	fn install_package_list(packages: &mut Vec<Package>, systemconfig: &Systemconfig) -> Result<(), ()> {
		for i in 0..packages.len() {
			let pack_opt = packages.get_mut(i);
			match pack_opt {

				Some(pack_ref) => {
					let res = Self::install_package(pack_ref, systemconfig);
					match res {
						Ok(()) => { }
						Err(err) => {
							return Err(err);
						}
					}
				}

				None => {
					return Err(())
				}

			}

		}

		Ok(())
	}
}