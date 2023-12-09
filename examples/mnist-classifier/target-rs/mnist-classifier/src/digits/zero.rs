use super::*;

#[ad_hoc_task_dependency::val_item_return_ref(24)]
pub fn open_one_match() -> FermiMatchResult {
    fermi_match(major_concave_components(), &vec![almost_closed])
}

pub fn almost_closed(cc: Leash<ConcaveComponent>) -> Option<f32> {
    require!(cc.angle_change() + 0.0f32 < -140.0f32);
    Some(-cc.angle_change() + 0.0f32)
}

#[ad_hoc_task_dependency::val_item(25)]
pub fn is_zero() -> OneVsAll {}