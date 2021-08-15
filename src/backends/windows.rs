use crate::backends::Installer;
use crate::packageconfig::Package;
use crate::systemconfig::Systemconfig;

struct WindowsBackend;

impl Installer for WindowsBackend {
    fn install_package(package: &mut Package, systemconfig: &Systemconfig) -> Result<(), ()> {
        Err(())
    }
    fn install_package_list(packages: &mut Vec<Package>, systemconfig: &Systemconfig) -> Result<(), ()> {
        Err(())
    }
}
