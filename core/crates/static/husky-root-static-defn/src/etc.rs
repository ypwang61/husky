use crate::*;

pub static TYPE_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "TypeType",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "Type",
        spatial_parameters: &[],
        static_trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Other,
        visual_ty: StaticVisualTy::Void,
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};
pub static TRAIT_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "TraitType",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "Trait",
        spatial_parameters: &[],
        static_trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Other,
        visual_ty: StaticVisualTy::Void,
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};
pub static MODULE_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "ModuleType",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "Module",
        spatial_parameters: &[],
        static_trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Other,
        visual_ty: StaticVisualTy::Void,
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};
