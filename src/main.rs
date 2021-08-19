use std::io::Write;

use clap::{App, Arg};

mod backends;
mod errors;
mod install;
mod packageconfig;
mod systemconfig;
mod tomlhelper;

fn print_section_separator() {
    println!("================================");
}

fn main() {
    let args = App::new("InstallABot")
        .version("1.0")
        .about("idk..something")
        .arg(Arg::with_name("platform").short("p").long("platform").help("specify the platform for the system config").takes_value(true))
        .arg(Arg::with_name("distro").short("d").long("distro").help("specify the distro for the linux platform config").takes_value(true))
        .arg(Arg::with_name("target").short("t").long("target").help("specify the target (containing the packagelist)").takes_value(true).required(true))
        .arg(Arg::with_name("interactive").short("i").long("interactive").help("interactive mode requiring confirmation for segments of the install process").takes_value(false))
        .arg(Arg::with_name("verify").long("verify").help("sanity check for the config file syntax").takes_value(false))
        .arg(Arg::with_name("dryrun").long("dryrun").help("run verification on the config file and also check for availability of local / remote resources").takes_value(false))
        .get_matches();

    let platform = args.value_of("platform");
    let distro = args.value_of("distro");
    let target = args.value_of("target").unwrap();
    let interactive = args.is_present("interactive");
    let verify = args.is_present("verify");
    let dryrun = args.is_present("dryrun");

    print_section_separator();
    let sys = match systemconfig::read_system_config(platform, distro) {
        Ok(cfg) => cfg,
        Err(e) => panic!("Error: {}", e),
    };
    println!(
        "System Configuration:\nName: {} | install_cmd: {}",
        sys.name.unwrap_or("default".to_string()),
        sys.install_cmd
    );
    if interactive {
        // continue?
    }

    let packageconfig = match packageconfig::PackageConfig::new() {
        Ok(cfg) => cfg,
        Err(e) => {
            panic!("Error loading packages config: {}", e);
        }
    };

    let targ_def: Vec<&str> = target.split(".").collect();
    let target_internal = match targ_def.len() {
        1 => packageconfig::Target {
            name: targ_def[0].to_string(),
            host: None,
        },
        2 => packageconfig::Target {
            name: targ_def[0].to_string(),
            host: Some(targ_def[1].to_string()),
        },
        _ => panic!(
            "invalid target definition: {}! allowed structure is TARGET or TARGET.HOST",
            target
        ),
    };
    print_section_separator();
    let packages = packageconfig.read_package_list(target_internal);
    println!("Found {} packages.", packages.len());

    // read package list
    if interactive {
        // show packagelist?
        print!("Show list of packages to install?\n(Y)es / (N)o: ");
        let _ = std::io::stdout().flush();
        let mut buff = String::new();
        let mut show_packages = false;
        match std::io::stdin().read_line(&mut buff) {
            Ok(_) => {
                let input = buff.trim();
                if input.eq_ignore_ascii_case("y") || input.eq_ignore_ascii_case("yes") {
                    show_packages = true;
                }
                ()
            }
            Err(e) => {
                println!("Error occured during read on stdin: {}", e);
                ()
            }
        };

        if show_packages {
            print_section_separator();
            println!("Packages:");
            for pack in &packages {
                println!("* {}", pack);
            }
        }
    }

    if verify {
        println!("Verify OK");
        return;
    }

    if dryrun {
        // execute dryrun
        println!("Dryrun OK");
        return;
    }

    // invoke installation
}
