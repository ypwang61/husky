#![feature(trait_upcasting)]
pub mod r#enum;
pub mod exception;
pub mod memo;
pub mod pedestal;
pub mod static_var;
pub mod r#struct;
#[cfg(test)]
mod tests;
pub mod ugly;
pub mod val;

pub use husky_standard_value::{
    frozen::ValueStands, value_conversion, FromValue, IntoValue, Value,
};

use self::pedestal::StandardPedestal;
use self::static_var::StandardStaticVarId;
use self::StandardLinketImpl as LinketImpl;
#[cfg(test)]
use self::StandardLinketImpl as __LinketImpl;
use husky_decl_macro_utils::for_all_ritchie_tys;
use husky_devsoul_interface::devsoul::IsDevsoulInterface;
use husky_item_path_interface::ItemPathIdInterface;
use husky_ki_repr_interface::KiReprInterface;
use husky_ki_repr_interface::{KiArgumentReprInterface, KiDomainReprInterface};
use husky_linket_impl::{
    eval_context::DevEvalContext,
    exception::TrackedException,
    impl_is_fn_linket_impl_source, impl_is_unveil_fn_linket_impl_source,
    linket_impl::{IsLinketImpl, LinketImplKiControlFlow, VmArgumentValue},
    pedestal::{IsPedestal, IsPedestalFull},
    static_var::StaticVarResult,
    ugly::__IsPedestal,
    LinketImplVmControlFlow, *,
};
use husky_standard_value::exception::Exception;
use husky_value_interface::ki_control_flow::KiControlFlow;
use husky_value_protocol::presentation::EnumUnitValuePresenter;
use linket_impl::{LinketImplTrackedExcepted, LinketImplTrackedExceptedValue};
use serde::{Deserialize, Serialize};

pub type StandardTrackedException = TrackedException<Exception, StandardPedestal>;
pub type StandardTrackedExcepted<T> = Result<T, TrackedException<Exception, StandardPedestal>>;
pub type StandardTrackedExceptedValue =
    Result<Value, TrackedException<Exception, StandardPedestal>>;

pub type StandardKiControlFlow<C = Value, B = Value> =
    KiControlFlow<C, B, StandardTrackedException>;

pub type StandardStaticVarResult<T> = StaticVarResult<StandardStaticVarId, T>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StandardLinketImpl {
    RitchieFn {
        /// it's the wrapper's responsibility to properly set ctx
        fn_ki_wrapper: fn(&[KiArgumentReprInterface]) -> StandardKiControlFlow,
        // todo: fn_vm_wrapper
        fn_pointer: fn(),
    },
    RitchieUnveilFn {
        /// it's the wrapper's responsibility to properly set ctx
        fn_wrapper: fn(&[KiArgumentReprInterface]) -> StandardKiControlFlow,
        fn_pointer: fn(),
    },
    RitchieGn {
        gn_ki_wrapper: fn(
            KiReprInterface,
            KiDomainReprInterface,
            StandardPedestal,
            &[KiArgumentReprInterface],
        ) -> StandardKiControlFlow,
    },
    // todo: this should be merged into RichieFn?
    EnumVariantConstructor {
        enum_variant_constructor_ki_wrapper: fn(&[KiArgumentReprInterface]) -> Value,
        enum_variant_constructor_vm_wrapper: fn(Vec<Value>) -> Value,
    },
    EnumVariantDestructor {
        enum_variant_destructor_wrapper: fn(Value) -> Vec<Value>,
    },
    EnumVariantDiscriminator {
        enum_variant_discriminator_wrapper: fn(Value) -> bool,
    },
    EnumVariantField {
        enum_variant_field_wrapper: fn(Value) -> Value,
    },
    /// used to get the json value of an enum u8-represented given only the index
    EnumUnitValuePresenter { presenter: EnumUnitValuePresenter },
    StructDestructor {
        struct_destructor_wrapper: fn(Value) -> Vec<Value>,
    },
    StructField {
        struct_field_wrapper: fn(Value) -> Value,
    },
    Val {
        init_item_path_id_interface: fn(ItemPathIdInterface),
        ki_wrapper: fn() -> StandardKiControlFlow,
    },
    Memo {
        init_item_path_id_interface: fn(ItemPathIdInterface),
        ki_wrapper: fn(Value) -> StandardKiControlFlow,
    },
    StaticVar {
        init_item_path_id_interface: fn(ItemPathIdInterface),
        get_id: fn() -> StandardStaticVarId,
        ids: for<'db> fn(
            &'db [ItemPathIdInterface],
        ) -> Box<dyn Iterator<Item = StandardStaticVarId> + 'db>,
        try_replace_id: unsafe fn(
            StandardStaticVarId,
            locked: &[ItemPathIdInterface],
        ) -> StandardStaticVarResult<Box<dyn FnOnce() + 'static>>,
    },
    // todo: memo
}

impl Copy for StandardLinketImpl {}

impl IsLinketImpl for StandardLinketImpl {
    type Pedestal = StandardPedestal;
    type Value = Value;
    type Exception = Exception;

    fn eval_ki(
        self,
        ki_repr_interface: KiReprInterface,
        ki_domain_repr_interface: KiDomainReprInterface,
        ki_argument_reprs: &[KiArgumentReprInterface],
        ctx: DevEvalContext<StandardLinketImpl>,
    ) -> StandardKiControlFlow {
        match self {
            StandardLinketImpl::RitchieFn { fn_ki_wrapper, .. } => fn_ki_wrapper(ki_argument_reprs),
            StandardLinketImpl::RitchieUnveilFn { fn_wrapper, .. } => fn_wrapper(ki_argument_reprs),
            StandardLinketImpl::RitchieGn { gn_ki_wrapper } => {
                let pedestal = ctx.eval_ki_pedestal(ki_repr_interface);
                gn_ki_wrapper(
                    ki_repr_interface,
                    ki_domain_repr_interface,
                    pedestal,
                    ki_argument_reprs,
                )
            }
            StandardLinketImpl::EnumVariantConstructor { .. } => todo!(),
            StandardLinketImpl::EnumVariantDestructor { .. } => todo!(),
            StandardLinketImpl::EnumVariantDiscriminator { .. } => todo!(),
            StandardLinketImpl::EnumVariantField { .. } => todo!(),
            StandardLinketImpl::EnumUnitValuePresenter { .. } => {
                unreachable!("this linket is not meant to be evaluated like this")
            }
            StandardLinketImpl::StructDestructor {
                struct_destructor_wrapper,
            } => todo!(),
            StandardLinketImpl::StructField {
                struct_field_wrapper,
            } => {
                debug_assert_eq!(ki_argument_reprs.len(), 1);
                let KiArgumentReprInterface::Simple(owner) = ki_argument_reprs[0] else {
                    unreachable!()
                };
                let owner = ctx.eval_ki_repr_interface(owner)?;
                StandardKiControlFlow::Continue(struct_field_wrapper(owner))
            }
            StandardLinketImpl::StaticVar { .. } => todo!(),
            StandardLinketImpl::Val { ki_wrapper, .. } => {
                debug_assert!(ki_argument_reprs.is_empty());
                ki_wrapper()
            }
            StandardLinketImpl::Memo {
                ki_wrapper,
                init_item_path_id_interface: set_item_path_id_interface,
            } => {
                debug_assert_eq!(ki_argument_reprs.len(), 1);
                let KiArgumentReprInterface::Simple(__self) = ki_argument_reprs[0] else {
                    unreachable!()
                };
                let __self = ctx.eval_ki_repr_interface(__self)?;
                ki_wrapper(__self)
            }
        }
    }

    fn eval_vm(
        self,
        arguments: Vec<VmArgumentValue<Self>>,
        db: &dyn std::any::Any,
    ) -> LinketImplVmControlFlow<Self> {
        todo!()
    }

    fn enum_index_value_presenter(self) -> EnumUnitValuePresenter {
        match self {
            StandardLinketImpl::EnumUnitValuePresenter { presenter } => presenter,
            _ => unreachable!(),
        }
    }

    fn init_item_path_id_interface(self, item_path_id_interface: ItemPathIdInterface) {
        match self {
            StandardLinketImpl::RitchieFn { .. } => (),
            StandardLinketImpl::RitchieUnveilFn { .. } => (),
            StandardLinketImpl::RitchieGn { .. } => (),
            StandardLinketImpl::EnumVariantConstructor { .. } => (),
            StandardLinketImpl::EnumVariantDestructor { .. } => (),
            StandardLinketImpl::EnumVariantDiscriminator { .. } => (),
            StandardLinketImpl::EnumVariantField { .. } => (),
            StandardLinketImpl::EnumUnitValuePresenter { .. } => (),
            StandardLinketImpl::StructDestructor { .. } => (),
            StandardLinketImpl::StructField { .. } => (),
            StandardLinketImpl::Val {
                init_item_path_id_interface,
                ..
            }
            | StandardLinketImpl::Memo {
                init_item_path_id_interface,
                ..
            }
            | StandardLinketImpl::StaticVar {
                init_item_path_id_interface,
                ..
            } => init_item_path_id_interface(item_path_id_interface),
        }
    }

    fn static_var_id(self) -> <Self::Pedestal as IsPedestal>::StaticVarId {
        let StandardLinketImpl::StaticVar { get_id, .. } = self else {
            unreachable!()
        };
        get_id()
    }

    fn with_static_var_id<R>(
        self,
        static_var_id: <Self::Pedestal as IsPedestal>::StaticVarId,
        locked: &[ItemPathIdInterface],
        f: impl FnOnce() -> R,
    ) -> StaticVarResult<<Self::Pedestal as IsPedestal>::StaticVarId, R> {
        let StandardLinketImpl::StaticVar {
            get_id,
            try_replace_id,
            ..
        } = self
        else {
            unreachable!()
        };
        unsafe {
            let restore = try_replace_id(static_var_id, locked)?;
            let r = f();
            restore();
            Ok(r)
        }
    }

    fn all_static_var_ids<'db>(
        self,
        locked: &'db [ItemPathIdInterface],
    ) -> Box<dyn Iterator<Item = StandardStaticVarId> + 'db> {
        let StandardLinketImpl::StaticVar { ids, .. } = self else {
            unreachable!()
        };
        ids(locked)
    }
}

pub struct FnLinketImplSource<Pedestal, DevsoulInterface, F>(
    pub std::marker::PhantomData<(Pedestal, DevsoulInterface)>,
    pub F,
);

for_all_ritchie_tys! {impl_is_fn_linket_impl_source}

pub struct UnveilFnLinketImplSource<Pedestal, DevsoulInterface, F>(
    pub std::marker::PhantomData<(Pedestal, DevsoulInterface)>,
    pub F,
);

for_all_ritchie_tys! {impl_is_unveil_fn_linket_impl_source}

pub trait IsGnItem {
    type LinketImpl: IsLinketImpl;

    fn generic_pedestal(
        specific_pedestal: <Self::LinketImpl as IsLinketImpl>::Pedestal,
    ) -> <Self::LinketImpl as IsLinketImpl>::Pedestal;

    type ValueAtGenericPedestal;

    /// compute `generic_pedestal` here for efficiency
    fn train(
        ki_domain_repr_interface: KiDomainReprInterface,
        ki_argument_repr_interfaces: &[KiArgumentReprInterface],
    ) -> LinketImplKiControlFlow<Self::LinketImpl, Self::ValueAtGenericPedestal>;

    type EvalOutput;

    fn eval(
        ki_argument_reprs: &[KiArgumentReprInterface],
        value_at_generic_pedestal: &Self::ValueAtGenericPedestal,
    ) -> Self::EvalOutput;
}

#[macro_export]
macro_rules! gn_linket_impl {
    ($gn_item: ty) => {{
        __LinketImpl::RitchieGn {
            gn_ki_wrapper: <$gn_item>::gn_ki_wrapper,
        }
    }};
}

#[test]
#[ignore]
fn gn_linket_impl_works() {
    todo!()
}

#[macro_export]
macro_rules! class_specific_leashed_field_into_value {
    (copyable $expr: expr) => {{
        (*$expr).into_value()
    }};
    (vec $expr: expr) => {{
        Leash::<[_]>::new($expr).into_value()
    }};
    (other $expr: expr) => {{
        Leash::new($expr).into_value()
    }};
}