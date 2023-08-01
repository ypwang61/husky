mod config;
mod form;
mod key;
mod table;

pub use config::*;
pub use form::*;
use husky_entity_path::ItemPath;
pub use key::*;
pub use table::*;

use husky_coword::Ident;
use husky_ethereal_term::EtherealTerm;
use husky_vm::__ResolvedLinkage;
use husky_vm::{Binding, EntityUid, __LinkageGroup};
use map_collect::MapCollect;
use std::collections::HashMap;
use sync_utils::ASafeRwLock;

pub trait ResolveLinkage {
    fn linkage_table(&self) -> &LinkageTable;

    fn index_linkage(&self, _opd_tys: Vec<EtherealTerm>) -> __LinkageGroup {
        todo!()
        // if let Some(linkage) = self
        //     .linkage_table()
        //     .element_access(opd_tys.map(|ty| self.item_uid(*ty)))
        // {
        //     linkage
        // } else {
        //     let this_ty_defn = self.item_defn(opd_tys[0]).unwrap();
        //     let std_ops_index_trai = self.route_call(
        //         self.item_route_menu().std_ops_index_trai,
        //         thin_vec![SpatialArgument::EntityRoute(opd_tys[1])],
        //     );
        //     let index_trai_impl = this_ty_defn.trait_impl(std_ops_index_trai).unwrap();
        //     match index_trai_impl.member_impls[1].variant {
        //         EntityDefnVariant::Method { ref opt_source, .. } => {
        //             if let Some(source) = opt_source {
        //                 match source {
        //                     CallFormSource::Static(linkage) => *linkage,
        //                     _ => todo!(),
        //                 }
        //             } else {
        //                 todo!()
        //             }
        //         }
        //         _ => {
        //             // p!(method_variant, this_ty_defn.file, this_ty_defn.range);
        //             panic!()
        //         }
        //     }
        // }
    }

    fn field_linkage(&self, _this_ty: EtherealTerm, _field_ident: Ident) -> Option<__LinkageGroup> {
        todo!()
        // if !this_ty.is_intrinsic() {
        //     panic!("expect intrinsic ty, but get `{}` instead", this_ty)
        // }
        // if let Some(linkage) = self
        //     .linkage_table()
        //     .field_linkage_source(self.item_uid(this_ty), field_ident)
        // {
        //     return Some(linkage);
        // }
        // let this_ty_defn = self.item_defn(this_ty).unwrap();
        // let ty_field_defn = this_ty_defn.field(field_ident);
        // match ty_field_defn.variant {
        //     EntityDefnVariant::TyField { opt_linkage, .. } => opt_linkage,
        //     _ => panic!(""),
        // }
    }

    fn field_linkage_resolved(
        &self,
        this_ty: EtherealTerm,
        field_ident: Ident,
        field_binding: Binding,
    ) -> Option<__ResolvedLinkage> {
        self.field_linkage(this_ty, field_ident)
            .map(|linkage| linkage.bind(field_binding))
    }

    fn method_linkage(&self, _method_route: EtherealTerm) -> Option<__LinkageGroup> {
        todo!()
        // opt_linkage_wrapper(
        //     &self.linkage_table().config,
        //     || {
        //         if let Some(linkage) = self
        //             .linkage_table()
        //             .routine_linkage(self.item_uid(method_route))
        //         {
        //             Some(linkage)
        //         } else {
        //             let method_defn = self.item_defn(method_route).unwrap();
        //             match method_defn.variant {
        //                 EntityDefnVariant::Builtin => todo!(),
        //                 EntityDefnVariant::Method { ref opt_source, .. } => {
        //                     if let Some(ref source) = opt_source {
        //                         match source {
        //                             CallFormSource::Func { .. }
        //                             | CallFormSource::Proc { .. }
        //                             | CallFormSource::Lazy { .. } => None,
        //                             CallFormSource::Static(linkage_source) => Some(*linkage_source),
        //                         }
        //                     } else {
        //                         todo!()
        //                         // ,
        //                         // MethodFnDefnKind::TraitMethod {
        //                         //     trai,
        //                         //     ref opt_default_source,
        //                         // } => opt_default_source.as_ref().map(|source| match source {
        //                         //     CallFormSource::Func { stmts } => todo!(),
        //                         //     CallFormSource::Proc { stmts } => todo!(),
        //                         //     CallFormSource::Lazy { stmts } => todo!(),
        //                         //     CallFormSource::Static(linkage_source) => *linkage_source,
        //                         // }),
        //                         // MethodFnDefnKind::TraitMethodImpl { trai, opt_source } => {
        //                         //     if let Some(source) = opt_source {
        //                         //         return match source {
        //                         //             CallFormSource::Func { ref stmts } => todo!(),
        //                         //             CallFormSource::Proc { ref stmts } => todo!(),
        //                         //             CallFormSource::Lazy { ref stmts } => todo!(),
        //                         //             CallFormSource::Static(linkage_source) => Some(*linkage_source),
        //                         //         };
        //                         //     }
        //                         //     let trai_syn_defn = self.item_defn(*trai).unwrap();
        //                         //     match trai_syn_defn.variant {
        //                         //         EntityDefnVariant::Trait {
        //                         //             ref template_parameters,
        //                         //             ref members,
        //                         //         } => {
        //                         //             let member = members
        //                         //                 .iter()
        //                         //                 .find(|member| member.ident == method_defn.ident)
        //                         //                 .unwrap();
        //                         //             match member.variant {
        //                         //                 EntityDefnVariant::Method {
        //                         //                     method_variant:
        //                         //                         MethodFnDefnKind::TraitMethod {
        //                         //                             trai,
        //                         //                             ref opt_default_source,
        //                         //                         },
        //                         //                     ..
        //                         //                 } => match opt_default_source.as_ref().unwrap() {
        //                         //                     CallFormSource::Func { ref stmts } => todo!(),
        //                         //                     CallFormSource::Proc { ref stmts } => todo!(),
        //                         //                     CallFormSource::Lazy { ref stmts } => todo!(),
        //                         //                     CallFormSource::Static(linkage_source) => {
        //                         //                         Some(*linkage_source)
        //                         //                     }
        //                         //                 },
        //                         //                 _ => panic!(),
        //                         //             }
        //                         //         }
        //                         //         _ => panic!(),
        //                         //     }
        //                         // }
        //                         // },
        //                     }
        //                 }
        //                 _ => {
        //                     p!(method_route);
        //                     todo!()
        //                 }
        //             }
        //         }
        //     },
        //     || format!("method `{method_route}`"),
        // )
    }

    fn routine_linkage(&self, _routine: EtherealTerm) -> Option<__LinkageGroup> {
        todo!()
        // opt_linkage_wrapper(
        //     &self.linkage_table().config,
        //     || match self.item_source(routine).unwrap() {
        //         EntitySource::StaticModuleItem(static_defn) => match static_defn.variant {
        //             EntityStaticDefnVariant::Function { linkage, .. } => Some(linkage),
        //             _ => todo!(),
        //         },
        //         EntitySource::StaticTypeMember(static_defn) => match static_defn.variant {
        //             EntityStaticDefnVariant::Method { opt_linkage, .. } => opt_linkage,
        //             _ => todo!(),
        //         },
        //         EntitySource::StaticTraitMember(_) => todo!(),
        //         EntitySource::StaticTraitForTypeMember => todo!(),
        //         EntitySource::WithinBuiltinModule => todo!(),
        //         EntitySource::WithinModule { .. } => self
        //             .linkage_table()
        //             .routine_linkage(self.item_uid(routine)),
        //         EntitySource::Module { .. } => todo!(),
        //         EntitySource::TargetInput => todo!(),
        //         EntitySource::Any { .. } => todo!(),
        //         EntitySource::StaticEnumVariant(_) => todo!(),
        //         EntitySource::ThisType { .. } => todo!(),
        //     },
        //     || format!("routine {routine}"),
        // )
    }

    fn type_call_linkage(&self, _ty: EtherealTerm) -> Option<__LinkageGroup> {
        todo!()
        // opt_linkage_wrapper(
        //     &self.linkage_table().config,
        //     || {
        //         if let Some(linkage) = self.linkage_table().type_call_linkage(self.item_uid(ty)) {
        //             return Some(linkage);
        //         }
        //         let type_defn = self.item_defn(ty).unwrap();
        //         match type_defn.variant {
        //             EntityDefnVariant::EtherealTerm {
        //                 ref opt_type_call, ..
        //             } => opt_type_call
        //                 .as_ref()
        //                 .map(|type_call| type_call.opt_linkage)
        //                 .flatten(),
        //             _ => panic!(),
        //         }
        //     },
        //     || format!("type call for type `{ty}`"),
        // )
    }

    fn feature_eager_block_linkage(&self, _item_path: ItemPath) -> Option<__LinkageGroup> {
        todo!()
        // opt_linkage_wrapper(
        //     &self.linkage_table().config,
        //     || {
        //         self.linkage_table()
        //             .feature_eager_block_linkage(self.item_uid(route))
        //     },
        //     || format!("eager block for feature `{route}`"),
        // )
    }
}

fn opt_linkage_wrapper(
    config: &LinkageTableConfig,
    f: impl FnOnce() -> Option<__LinkageGroup>,
    message: impl FnOnce() -> String,
) -> Option<__LinkageGroup> {
    let opt_linkage = f();
    if config.warn_missing_linkage {
        if opt_linkage.is_none() {
            use husky_print_utils::*;
            println!(
                "{YELLOW}[warning] {RED}expect linkage{RESET} for {GREEN}{}{RESET}",
                message()
            )
        }
    }
    opt_linkage
}