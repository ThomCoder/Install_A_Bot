use crate::backends::Installer;
use crate::packageconfig::Package;

pub fn install_packages(packages: Vec<Package>, backend: &impl Installer) {}
