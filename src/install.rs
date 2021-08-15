use crate::backends::Installer;
use crate::parser::Package;

pub fn install_packages(packages: Vec<Package>, backend: &impl Installer) {}
