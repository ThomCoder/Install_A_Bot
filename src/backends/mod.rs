use crate::parser::Package;

pub mod linux;
pub mod mac;
pub mod windows;

pub trait Installer {
    fn install_package(package: &Package);
    fn install_package_list(packages: Vec<Package>);
}
