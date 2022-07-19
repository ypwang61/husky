#![allow(dead_code, warnings)]
#![feature(unboxed_closures)]
mod try_as;
#[cfg(test)] // this crate is for trying stuffs
mod try_atomic;
mod try_control_flow;
mod try_diff_text;
mod try_eq;
mod try_fat_pointer;
mod try_fn;
mod try_mut_ref;
mod try_random;
mod try_rayon;
mod try_ref;
#[cfg(test)]
mod try_salsa;
mod try_serde;
mod try_size;
mod try_trait;
mod try_xxhash;

use check_utils::*;
use print_utils::*;
