use crate::backends::Installer;
use crate::errors::{Error, ErrorCode};
use crate::packageconfig::Package;
use crate::systemconfig::Systemconfig;

struct MacBackend;

impl Installer for MacBackend {
    fn install_package(package: &mut Package, systemconfig: &Systemconfig) -> Result<(), Error> {
        return Err(Error::with_msg(
            ErrorCode::InvalidParameter,
            "Mac backend not yet supported".to_string(),
        ));
    }

    fn install_package_list(
        packages: &mut Vec<Package>,
        systemconfig: &Systemconfig,
    ) -> Result<(), Error> {
        return Err(Error::with_msg(
            ErrorCode::InvalidParameter,
            "Mac backend not yet supported".to_string(),
        ));
    }
}
