use crate::packageconfig::Package;
use crate::systemconfig::Systemconfig;

pub mod linux;
pub mod mac;
pub mod windows;

pub trait Installer {
    fn install_package(package: &Package, systemconfig: &Systemconfig);
    fn install_package_list(packages: Vec<Package>, systemconfig: &Systemconfig);
}
