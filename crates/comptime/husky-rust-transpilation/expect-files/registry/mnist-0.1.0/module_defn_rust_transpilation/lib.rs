#![allow(warnings, non_snake_case)]
use husky_core::*;
::ad_hoc_task_dependency::init_crate!();

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MnistLabel {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryImage28([u32; 30]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryGrid28([u32; 31]);

#[ad_hoc_task_dependency::val_item_return_ref(0)]
pub fn input() -> BinaryImage28 {}

impl BinaryImage28 {
    
}

impl BinaryGrid28 {
    
}

