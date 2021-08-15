use clap::{App, Arg};

mod backends;
mod install;
mod packageconfig;
mod systemconfig;

fn main() {
    let args = App::new("InstallABot")
        .version("1.0")
        .about("idk..something")
        .arg(Arg::with_name("platform").short("p").long("platform").help("specify the platform for the system config").takes_value(true))
        .arg(Arg::with_name("distro").short("d").long("distro").help("specify the distro for the linux platform config").takes_value(true))
        .arg(Arg::with_name("target").short("t").long("target").help("specify the target (containing the packagelist)").takes_value(true))
        .arg(Arg::with_name("interactive").short("i").long("interactive").help("interactive mode requiring confirmation for segments of the install process").takes_value(false))
        .arg(Arg::with_name("verify").long("verify").help("sanity check for the config file syntax").takes_value(false))
        .arg(Arg::with_name("dryrun").long("dryrun").help("run verification on the config file and also check for availability of local / remote resources").takes_value(false))
        .get_matches();

    let platform = args.value_of("platform");
    let distro = args.value_of("distro");
    let interactive = args.is_present("interactive");
    let verify = args.is_present("verify");
    let dryrun = args.is_present("dryrun");

    let sys = systemconfig::read_system_config("packages.toml", platform, distro).unwrap();
    println!(
        "System Configuration:\nName: {} | install_cmd: {}",
        sys.name.unwrap_or("default".to_string()),
        sys.install_cmd
    );
    if interactive {
        // continue?
    }

    // read package list
    if interactive {
        // show packagelist?
        // if yes -> print packages
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
