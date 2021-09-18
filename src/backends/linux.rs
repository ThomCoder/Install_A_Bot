use crate::backends::Installer;
use crate::packageconfig::Package;
use crate::packageconfig::Status;
use crate::packageconfig::Source;
use crate::systemconfig::Systemconfig;
use std::process::Command;

pub struct LinuxBackend;

fn handle_regular_install(package: &mut Package, systemconfig: &Systemconfig) -> Result<(), ()> {
	dbg!("handle_regular_install");
	dbg!(systemconfig.install_cmd.clone());
	dbg!(package.name.clone());

	let split_cmd = systemconfig.install_cmd.split_whitespace();
	let parts: Vec<&str> = split_cmd.collect();

	// Extract the actual arguments from the full install cmd
	let mut args: Vec<&str> = parts.clone();
	args.remove(0);

	let output = Command::new(parts[0])
		.args(args)
		.arg(package.name.clone())
		.output()
		.unwrap();

	if output.status.success() {
		package.status = Status::Succeeded;
		Ok(())
	} else {
		package.status = Status::Failed;
		dbg!(output.status.to_string());
		Err(())
	}
}

// TODO: This function and handle_regular_install have a lot of code in common.
// Refactor to reduce code duplication.
fn handle_local_install(package: &mut Package, systemconfig: &Systemconfig) -> Result<(), ()> {
	dbg!("handle_local_install!");
	dbg!(systemconfig.install_cmd.clone());
	dbg!(package.name.clone());
	dbg!(package.source.as_ref().unwrap().0.clone());
	dbg!(package.source.as_ref().unwrap().1.clone());
	dbg!(package.cmd.as_ref());

	let split_cmd = systemconfig.install_cmd.split_whitespace();
	let parts: Vec<&str> = split_cmd.collect();

	// Extract the actual arguments from the full install cmd
	let mut args: Vec<&str> = parts.clone();
	args.remove(0);

	let output = Command::new(parts[0])
		.args(args)
		.arg(package.source.as_ref().unwrap().1.clone())
		.output()
		.unwrap();

	if output.status.success() {
		package.status = Status::Succeeded;
		Ok(())
	} else {
		package.status = Status::Failed;
		dbg!(output.status.to_string());
		Err(())
	}
}

impl Installer for LinuxBackend {
	fn install_package(package: &mut Package, systemconfig: &Systemconfig) -> Result<(), ()> {
		match package.source.as_ref() {
			Some(_) => {

				let src_enum = package.source.as_ref().unwrap().0.clone();
				match src_enum {
					Source::Git => {
						Err(())
					}

					Source::Web => {
						Err(())
					}

					Source::Local => {
						return handle_local_install(package, systemconfig);
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