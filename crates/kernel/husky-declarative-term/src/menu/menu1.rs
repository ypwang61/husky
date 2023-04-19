use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct DeclarativeTermMenu1 {
    parent: DeclarativeTermMenu0,
    eval_ref_ty: DeclarativeTerm,
    static_ref_ty: DeclarativeTerm,
    explicit_invariant_ty0_to_trai_ty: DeclarativeTermCurry,
    explicit_covariant_ty0_to_ty0: DeclarativeTermCurry,
    explicit_contravariant_ty0_to_ty0: DeclarativeTermCurry,
    explicit_invariant_ty0_to_ty0: DeclarativeTermCurry,
}

impl std::ops::Deref for DeclarativeTermMenu1 {
    type Target = DeclarativeTermMenu0;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DeclarativeTermMenu1 {
    pub fn new(
        db: &dyn DeclarativeTermDb,
        _toolchain: Toolchain,
        menu0: DeclarativeTermMenu0,
    ) -> Self {
        // todo!()
        Self {
            eval_ref_ty: DeclarativeTermExplicitApplication::new(
                db,
                menu0.ref_ty_path(),
                menu0.eval_lifetime(),
            )
            .into(),
            static_ref_ty: DeclarativeTermExplicitApplication::new(
                db,
                menu0.ref_ty_path(),
                menu0.static_lifetime(),
            )
            .into(),
            explicit_invariant_ty0_to_trai_ty: DeclarativeTermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Invariant,
                None,
                menu0.ty0().into(),
                menu0.trai_ty().into(),
            ),
            explicit_covariant_ty0_to_ty0: DeclarativeTermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Covariant,
                None,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            explicit_contravariant_ty0_to_ty0: DeclarativeTermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Contravariant,
                None,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            explicit_invariant_ty0_to_ty0: DeclarativeTermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Invariant,
                None,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            parent: menu0,
        }
    }

    pub fn eval_ref_ty(&self) -> DeclarativeTerm {
        self.eval_ref_ty
    }

    pub fn static_ref_ty(&self) -> DeclarativeTerm {
        self.static_ref_ty
    }

    pub fn invariant_ty0_to_trai_ty(&self) -> DeclarativeTermCurry {
        self.explicit_invariant_ty0_to_trai_ty
    }

    pub fn explicit_covariant_ty0_to_ty0(&self) -> DeclarativeTermCurry {
        self.explicit_covariant_ty0_to_ty0
    }

    pub fn explicit_contravariant_ty0_to_ty0(&self) -> DeclarativeTermCurry {
        self.explicit_contravariant_ty0_to_ty0
    }

    pub fn ex_inv_ty0_to_ty0(&self) -> DeclarativeTermCurry {
        self.explicit_invariant_ty0_to_ty0
    }
}