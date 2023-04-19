#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(trait_upcasting)]
mod card;
mod data;
mod db;
mod engine;
mod expectation;
mod nested;
mod progress;
mod region;
mod resolve;
mod substitution;
mod term;
#[cfg(test)]
mod tests;
mod ty_info;
mod utils;

pub use self::card::*;
pub use self::data::*;
pub use self::db::*;
pub use self::engine::*;
pub use self::expectation::*;
pub use self::progress::*;
pub use self::region::*;
pub use self::resolve::*;
pub use self::substitution::*;
pub use self::term::*;
pub use self::ty_info::*;

#[cfg(test)]
pub(crate) use self::tests::*;

use self::nested::*;
use either::*;
use husky_declarative_signature::*;
use husky_entity_path::*;
use husky_ethereal_term::*;
use husky_expr::*;
use husky_print_utils::p;
use husky_term_prelude::*;
use salsa::DebugWithDb as _;
use smallvec::*;

#[salsa::jar(db = FluffyTermDb)]
pub struct FluffyTermJar(term_ritchie_fluffy_data, term_application_fluffy_data);