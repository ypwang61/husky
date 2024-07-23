use crate::*;
use husky_wild_utils::arb_ref;
use lazy_static::lazy_static;
use shifted_unsigned_int::ShiftedU32;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
    sync::{
        atomic::{AtomicU32, Ordering},
        RwLock,
    },
};

pub struct Seq<T>(ShiftedU32, PhantomData<T>)
where
    T: Any + Send + Sync;

#[derive(Default)]
pub struct SeqStorage(HashMap<(TypeId, ShiftedU32), Box<dyn Any + Send + Sync>>);

lazy_static! {
    static ref SEQ_STORAGE: RwLock<SeqStorage> = Default::default();
}

static NEXT_ID: AtomicU32 = AtomicU32::new(0);

impl<T> Seq<T>
where
    T: Any + Send + Sync,
{
    pub fn new(data: Vec<T>) -> Self {
        let mut seq_storage_guard = SEQ_STORAGE.write().unwrap();
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
        let id = ShiftedU32::from(id);
        seq_storage_guard
            .0
            .insert((TypeId::of::<T>(), id), Box::new(data));
        Seq(id, PhantomData)
    }
}

#[macro_export]
macro_rules! seq {
    ($($elements: expr),*) => {{
        Seq::new(vec![$($elements),*])
    }};
}

impl<T> Seq<T>
where
    T: Any + Send + Sync,
{
    pub fn slice(self) -> &'static [T] {
        let seq_storage_guard = SEQ_STORAGE.read().unwrap();
        let a: &Vec<T> = seq_storage_guard
            .0
            .get(&(TypeId::of::<T>(), self.0))
            .unwrap()
            .downcast_ref()
            .unwrap();
        unsafe { arb_ref(a) }
    }
}

impl<T> Clone for Seq<T>
where
    T: Any + Send + Sync,
{
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

impl<T> Copy for Seq<T> where T: Any + Send + Sync {}

impl<T> std::fmt::Debug for Seq<T>
where
    T: Any + Send + Sync + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.slice()).finish()
    }
}

#[test]
fn seq_debug_works() {
    let seq = seq![1, 2, 3, 4];
    expect![[r#"
        [
            1,
            2,
            3,
            4,
        ]
    "#]]
    .assert_debug_eq(&seq);
}

impl<T1, T2> Seq<(T1, T2)>
where
    T1: Any + Send + Sync,
    T2: Any + Send + Sync,
{
    fn decouple(self) -> (Seq<T1>, Seq<T2>) {
        todo!()
    }
}

impl<T1, T2, T3> Seq<(T1, T2, T3)>
where
    T1: Any + Send + Sync,
    T2: Any + Send + Sync,
    T3: Any + Send + Sync,
{
    fn decouple(self) -> (Seq<T1>, Seq<T2>, Seq<T3>) {
        todo!()
    }
}

#[test]
fn seq_decouple_works() {
    let seq = Seq::new(vec![(1, 1)]);
    // seq.decouple();
    print!("{:?}", seq);
}
