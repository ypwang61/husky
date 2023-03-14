mod ill_formed;
mod registry;
mod ty;
mod ty_as_trai;

pub use self::ill_formed::*;
pub use self::registry::*;
pub use self::ty::*;
pub use self::ty_as_trai::*;

use crate::*;
use husky_token::*;
use husky_word::IdentPairMap;
use parsec::ParseContext;
use thiserror::Error;
use vec_like::VecPairMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
#[enum_class::from_variants]
pub enum ImplBlock {
    Type(TypeImplBlock),
    TypeAsTrait(TypeAsTraitImplBlock),
    IllFormed(IllFormedImplBlock),
}

#[derive(Debug, Default, PartialEq, Eq)]
// #[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct ImplBlockBundle {
    all_ty_impl_blocks: Vec<TypeImplBlock>,
    all_ty_as_trai_impl_blocks: Vec<TypeAsTraitImplBlock>,
    all_ill_formed_impl_blocks: Vec<IllFormedImplBlock>,
}

impl ImplBlockBundle {
    pub fn all_ty_impl_blocks(&self) -> &[TypeImplBlock] {
        self.all_ty_impl_blocks.as_ref()
    }

    pub fn all_ty_as_trai_impl_blocks(&self) -> &[TypeAsTraitImplBlock] {
        self.all_ty_as_trai_impl_blocks.as_ref()
    }

    pub fn all_ill_formed_impl_blocks(&self) -> &[IllFormedImplBlock] {
        self.all_ill_formed_impl_blocks.as_ref()
    }
}

impl ImplBlock {
    pub fn id(self, db: &dyn EntityTreeDb) -> ImplBlockId {
        match self {
            ImplBlock::Type(impl_block) => impl_block.id(db).into(),
            ImplBlock::TypeAsTrait(impl_block) => impl_block.id(db).into(),
            ImplBlock::IllFormed(impl_block) => impl_block.id(db).into(),
        }
    }
}

impl ImplBlock {
    pub(crate) fn parse_from_token_group<'a>(
        db: &dyn EntityTreeDb,
        registry: &mut ImplBlockRegistry,
        module_symbol_context: ModuleSymbolContext<'a>,
        module_path: ModulePath,
        ast_idx: AstIdx,
        body: AstIdxRange,
        token_stream: TokenStream<'a>,
        princiapl_entity_path_expr_arena: &mut MajorPathExprArena,
    ) -> Self {
        let mut parser = MajorPathExprParser::new(
            db,
            token_stream,
            princiapl_entity_path_expr_arena,
            module_symbol_context,
        );
        parser.parse::<ImplToken>().unwrap().unwrap();
        if let Some(_) = parser.try_parse::<LeftAngleBracketOrLessThanToken>() {
            match ignore_implicit_parameters(&mut parser) {
                Ok(_) => (),
                Err(_e) => todo!(),
            }
        }
        let (_expr, path) = match parser.parse_principal_path_expr() {
            Ok((expr, path)) => (expr, path),
            Err(e) => {
                return IllFormedImplBlock::new(
                    db,
                    registry,
                    module_path,
                    ast_idx,
                    body,
                    ImplBlockIllForm::MajorPath(e),
                )
                .into();
            }
        };
        match path {
            ModuleItemPath::Type(ty) => {
                TypeImplBlock::new(db, registry, module_path, ast_idx, body, ty).into()
            }
            ModuleItemPath::Trait(_) => {
                todo!();

                new_impl(db, registry, module_path, ast_idx, body, todo!())
            }
            ModuleItemPath::Form(_) => todo!(),
        }
    }

    pub fn module_path(&self, _db: &dyn EntityTreeDb) -> ModulePath {
        todo!()
        // self.id(db).module_path
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
#[enum_class::from_variants]
pub enum ImplBlockId {
    Type(TypeImplBlockId),
    TypeAsTrait(TypeAsTraitImplBlockId),
    IllFormed(IllFormedImplBlockId),
}

impl ImplBlockId {
    pub fn module(self) -> ModulePath {
        match self {
            ImplBlockId::Type(id) => id.module(),
            ImplBlockId::TypeAsTrait(id) => id.module(),
            ImplBlockId::IllFormed(id) => id.module(),
        }
    }
}

fn new_impl(
    _db: &dyn EntityTreeDb,
    _registry: &mut ImplBlockRegistry,
    _module_path: ModulePath,
    _ast_idx: ArenaIdx<Ast>,
    _body: ArenaIdxRange<Ast>,
    _variant: (),
) -> ImplBlock {
    // let impl_kind = variant.kind();
    todo!()
    // ImplBlock::new(
    //     db,
    //     ImplId {
    //         module_path,
    //         impl_kind,
    //         disambiguator: registry.issue_disambiguitor(module_path, impl_kind),
    //     },
    //     ast_idx,
    //     body,
    //     variant,
    // )
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum ImplError {
    #[error("unmatched angle bras")]
    UnmatchedAngleBras,
    #[error("token error")]
    Token(#[from] TokenError),
    #[error("principal path expr error")]
    MajorPath(#[from] MajorPathExprError),
}

pub type ImplResult<T> = Result<T, ImplError>;

fn ignore_implicit_parameters<'a>(token_stream: &mut TokenStream<'a>) -> ImplResult<()> {
    let layer = 1;
    while let Some(token) = token_stream.next() {
        match token {
            Token::Punctuation(_) => todo!(),
            Token::Error(e) => return Err(e.clone().into()),
            _ => (),
        }
    }
    match layer {
        0 => Ok(()),
        _ => Err(ImplError::UnmatchedAngleBras),
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn ty_impl_blocks(
    db: &dyn EntityTreeDb,
    ty: TypePath,
) -> EntityTreeBundleResult<Vec<TypeImplBlock>> {
    let crate_path = ty.module_path(db).crate_path(db);
    let entity_tree_crate_bundle = db.entity_tree_crate_bundle(crate_path)?;
    Ok(entity_tree_crate_bundle
        .impl_blocks()
        .all_ty_impl_blocks
        .iter()
        .copied()
        .filter_map(|impl_block| (impl_block.ty(db) == ty).then_some(impl_block))
        .collect())
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn ty_items(
    db: &dyn EntityTreeDb,
    path: TypePath,
) -> EntityTreeBundleResult<IdentPairMap<AssociatedItem>> {
    let crate_path = path.module_path(db).crate_path(db);
    let entity_tree_crate_bundle = db.entity_tree_crate_bundle(crate_path)?;
    Ok(entity_tree_crate_bundle
        .impl_blocks()
        .all_ty_impl_blocks()
        .iter()
        .copied()
        .map(|impl_block| {
            ty_impl_block_items(db, impl_block)
                .iter()
                .map(|(ident, associated_item)| (*ident, *associated_item))
        })
        .flatten()
        .collect())
}
