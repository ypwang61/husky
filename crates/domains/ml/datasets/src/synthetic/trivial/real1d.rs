use crate::{synthetic::SimpleSyntheticDataset, *};
use std::sync::Arc;
use vm::{BoxedValue, Linkage, OutputContract, StackValue};
use xrng::XRng;

pub const SCOPE_DATA: &StaticEntityDefn = &StaticEntityDefn {
    subscopes: &[
        ("dataset1", DATASET1_SCOPE_DATA),
        ("dataset2", DATASET2_SCOPE_DATA),
    ],
    decl: StaticEntityDecl::Module,
};

pub const DATASET1_SCOPE_DATA: &StaticEntityDefn = &StaticEntityDefn {
    subscopes: &[],
    decl: StaticEntityDecl::Func(StaticCallDecl {
        generic_placeholders: &[],
        inputs: vec![],
        output_ty: "Dataset<f32, i32>",
        output_contract: OutputContract::Pure,
        linkage: Linkage {
            call: |_| Ok(StackValue::Boxed(BoxedValue::new(dataset1()))),
            nargs: 0,
        },
    }),
};

pub const DATASET2_SCOPE_DATA: &StaticEntityDefn = &StaticEntityDefn {
    subscopes: &[],
    decl: StaticEntityDecl::Func(StaticCallDecl {
        generic_placeholders: &[],
        inputs: vec![],
        output_ty: "Dataset<f32, i32>",
        output_contract: OutputContract::Pure,
        linkage: Linkage {
            call: |_| Ok(StackValue::Boxed(BoxedValue::new(dataset2()))),
            nargs: 0,
        },
    }),
};

pub fn gen_sample1<'eval>(seed: u64, idx: usize) -> LabeledData<'eval> {
    let mut xrng = XRng::new(((seed + (idx as u64)) >> 32) & ((idx as u64) << 32));
    if xrng.with_probability(0.5) {
        LabeledData {
            input: Arc::new(1.0f32),
            label: 1,
        }
    } else {
        LabeledData {
            input: Arc::new(-1.0f32),
            label: 1,
        }
    }
}

pub fn gen_sample2<'eval>(seed: u64, idx: usize) -> LabeledData<'eval> {
    let mut xrng = XRng::new(((seed + (idx as u64)) >> 32) & ((idx as u64) << 32));
    if xrng.with_probability(0.5) {
        LabeledData {
            input: Arc::new(1.0f32),
            label: 1,
        }
    } else {
        LabeledData {
            input: Arc::new(-1.0f32),
            label: 1,
        }
    }
}

pub fn dataset1<'eval>() -> Dataset<'eval> {
    Dataset::new(SimpleSyntheticDataset::new(1223418012u64, gen_sample1))
}

pub fn dataset2<'eval>() -> Dataset<'eval> {
    Dataset::new(SimpleSyntheticDataset::new(1213148012u64, gen_sample2))
}
