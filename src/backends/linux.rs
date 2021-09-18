use crate::backends::Installer;
use crate::packageconfig::Package;
use crate::systemconfig::Systemconfig;
use std::process::Command;

pub struct LinuxBackend;

fn handle_regular_install(package: &mut Package, systemconfig: &Systemconfig) -> Result<(), ()> {
	dbg!(systemconfig.install_cmd.clone());
	dbg!(package.name.clone());

	// Get a full fledged "String" so we can better work with it
	let install_cmd = systemconfig.install_cmd.to_string();
	let split = install_cmd.split_whitespace();

	let parts: Vec<&str> = split.collect();
	let mut args: Vec<&str> = parts.clone();
	args.remove(0);

	let output = Command::new(parts[0])
		.args(args)
		.arg(package.name.clone())
		.output()
		.unwrap();

	if output.status.success() {
		Ok(())
	} else {
		dbg!(output.status.to_string());
		Err(())
	}
}

impl Installer for LinuxBackend {
	fn install_package(package: &mut Package, systemconfig: &Systemconfig) -> Result<(), ()> {
		match package.source.as_ref() {
			Some(src) => {

				match src {
					Git => {
						Err(())
					}

					Web => {
						Err(())
					}

					Local => {
						Err(())
					}
				}

			}

			None => {
				return handle_regular_install(package, systemconfig)
			}
		}
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