use crate::packageconfig::Package;
use crate::systemconfig::Systemconfig;

pub mod linux;
pub mod mac;
pub mod windows;

pub trait Installer {
    fn install_package(package: &mut Package, systemconfig: &Systemconfig) -> Result<(), ()>;
    fn install_package_list(packages: &mut Vec<Package>, systemconfig: &Systemconfig) -> Result<(), ()>;
}
