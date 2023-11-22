use crate::*;
use husky_linkage::linkage::Linkage;
use husky_vfs::linktime_target_path::LinktimeTargetPath;

pub trait IsLinktime<ComptimeDb>: Sized + Send {
    type LinkageImpl: IsLinkageImpl;
    // linktime has the responsibility to guarantee that the linkage provided is up to date.
    fn get_linkage(&self, path: Linkage, db: &ComptimeDb) -> Self::LinkageImpl;
    fn new_linktime(target_path: LinktimeTargetPath, db: &ComptimeDb) -> Self;
}

pub trait IsLinkageImpl: Send + Copy {
    type Value;
    fn eval_fn() -> Self::Value;
    fn eval_gn() -> Self::Value;
}