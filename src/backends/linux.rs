use crate::backends::Installer;
use crate::errors::{Error, ErrorCode};
use crate::packageconfig::Package;
use crate::packageconfig::Status;
use crate::packageconfig::Source;
use crate::systemconfig::Systemconfig;
use std::process::Command;
use curl::easy::Easy;
use std::fs::File;
use std::io::Write;

pub struct LinuxBackend;

fn handle_regular_install(package: &mut Package, systemconfig: &Systemconfig) -> Result<(), Error> {
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
		return Err(Error::with_msg(
			ErrorCode::HostError,
			"Interaction with host package manager failed".to_string()
		));
	}
}

// TODO: This function and handle_regular_install have a lot of code in common.
// Refactor to reduce code duplication.
fn handle_local_install(package: &mut Package, systemconfig: &Systemconfig) -> Result<(), Error> {
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
		return Err(Error::with_msg(
			ErrorCode::HostError,
			"Interaction with host package manager failed".to_string()
		));
	}
}

fn handle_web_install(package: &mut Package, systemconfig: &Systemconfig) -> Result<(), Error> {
	dbg!("handle_web_install!");
	dbg!(systemconfig.install_cmd.clone());
	dbg!(package.name.clone());
	dbg!(package.source.as_ref().unwrap().0.clone());
	dbg!(package.source.as_ref().unwrap().1.clone());
	dbg!(package.cmd.as_ref());

	// Extract filename from url
	let source_dict = package.source.as_ref().unwrap();
	let source_owned = source_dict.to_owned();
	let url = source_owned.1.clone();
	let file_name = url.split('/').last().unwrap_or("foo.bin");

	let file = File::create(file_name);

	match file {
		Ok(mut f) => {
			let mut easy = Easy::new();
			easy.url(package.source.as_ref().unwrap().1.clone().as_str()).unwrap();

			easy.write_function(move |data| {
				let _ = f.write_all(data);
				Ok(data.len())
			}).unwrap();

			easy.perform().unwrap();

			println!("{}", easy.response_code().unwrap());
		}

		Err(e) => {
			dbg!(e);
			return Err(Error::with_msg(
				ErrorCode::NetworkError,
				"Unable to make network request for webinstall".to_string()
			));
		}
	}

	let split_cmd = systemconfig.install_cmd.split_whitespace();
	let parts: Vec<&str> = split_cmd.collect();

	// Extract the actual arguments from the full install cmd
	let mut args: Vec<&str> = parts.clone();
	args.remove(0);

	let output = Command::new(parts[0])
		.args(args)
		.arg(file_name)
		.output()
		.unwrap();

	if output.status.success() {
		package.status = Status::Succeeded;
		Ok(())
	} else {
		package.status = Status::Failed;
		dbg!(output.status.to_string());
		return Err(Error::with_msg(
			ErrorCode::HostError,
			"Interaction with host package manager failed".to_string()
		));
	}
}

impl Installer for LinuxBackend {
	fn install_package(package: &mut Package, systemconfig: &Systemconfig) -> Result<(), Error> {
		match package.source.as_ref() {
			Some(_) => {

				let src_enum = package.source.as_ref().unwrap().0.clone();
				match src_enum {
					Source::Git => {
						return Err(Error::with_msg(
							ErrorCode::InvalidParameter,
							"Git based install not yet supported".to_string()
						));
					}

					Source::Web => {
						return handle_web_install(package, systemconfig);
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

	fn install_package_list(packages: &mut Vec<Package>, systemconfig: &Systemconfig)
		-> Result<(), Error> {
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
					return Err(Error::with_msg(
						ErrorCode::InvalidParameter,
						"Received package option as None".to_string()
					));
				}

			}

		}

		Ok(())
	}
}