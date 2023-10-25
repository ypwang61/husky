use super::*;
use husky_entity_path::MajorEntityPath;
use husky_entity_syn_tree::EntitySynTreeError;
use idx_arena::{Arena, ArenaIdx};
use parsec::IsStreamParser;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum SynPrincipalEntityPathExpr {
    Root {
        path_name_token: PathNameRegionalToken,
        principal_entity_path: PrincipalEntityPath,
    },
    Subitem {
        parent: SynPrincipalEntityPathExprIdx,
        colon_colon_token: ColonColonRegionalToken,
        ident_token: SynExprResult<IdentRegionalToken>,
        path: SynExprResult<PrincipalEntityPath>,
    },
}

pub type SynPrincipalEntityPathExprArena = Arena<SynPrincipalEntityPathExpr>;
pub type SynPrincipalEntityPathExprIdx = ArenaIdx<SynPrincipalEntityPathExpr>;