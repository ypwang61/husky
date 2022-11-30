mod db;
mod jar;
mod menu;
mod name;

pub use db::*;
pub use jar::*;
pub use menu::*;

use husky_toolchain::Toolchain;
use husky_word::Identifier;
use name::package_name;
use semver::Version;
use std::path::PathBuf;
use url::Url;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PackagePathData {
    Builtin {
        ident: Identifier,
        toolchain: Toolchain,
    },
    Global {
        version: Version,
    },
    Local(PathBuf),
    Git(Url),
}

#[salsa::interned(jar = PackagePathJar)]
pub struct PackagePath {
    #[return_ref]
    data: PackagePathData,
}
