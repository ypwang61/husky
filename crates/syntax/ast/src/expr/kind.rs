use std::sync::Arc;

use syntax_types::Opr;
use text::Row;
use vm::PrimitiveValue;
use word::WordPtr;

use crate::*;
use entity_route::{EntityKind, EntityRouteKind, EntityRoutePtr, RangedEntityRoute};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawExprVariant {
    Variable {
        varname: CustomIdentifier,
        init_row: Row,
    },
    This {
        ty: Option<EntityRoutePtr>,
    },
    Unrecognized(CustomIdentifier),
    Entity {
        route: EntityRoutePtr,
        kind: EntityKind,
    },
    PrimitiveLiteral(PrimitiveValue),
    Bracketed(RawExprIdx),
    Opn {
        opr: Opr,
        opds: RawExprRange,
    },
    Lambda(
        Vec<(CustomIdentifier, Option<RangedEntityRoute>)>,
        RawExprIdx,
    ),
}
