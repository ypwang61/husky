#[cfg(feature = "item_path")]
pub mod item_path;
#[cfg(feature = "query")]
pub mod query;
#[cfg(feature = "registration")]
pub mod registration;

use convert_case::{Case, Casing};
// use husky_ethereal_term::*;use husky_ethereal_ty::*;
use std::fmt::Display;

pub struct BuildCodeGenStart;

impl Display for BuildCodeGenStart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"// this is generated by husky_vm_interface_code_gen::rust_code::write_rust_code
// do not modify by hand

use crate::*;

type void = ();
type r32 = u32;
type b64 = u64;
"#
        )
    }
}