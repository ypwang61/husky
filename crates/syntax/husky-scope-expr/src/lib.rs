mod error;

pub use self::error::*;

use husky_opn_syntax::Bracket;
use husky_scope::Scope;
use husky_token::*;
use husky_vfs::{ModulePath, VfsDb};
use parsec::{HasStreamState, StreamParser};

#[derive(Debug, PartialEq, Eq)]
pub struct VisibilityExpr {
    visibility: Scope,
    variant: VisibilityExprVariant,
}

impl VisibilityExpr {
    #[inline(always)]
    pub fn new_protected(module_path: ModulePath) -> Self {
        VisibilityExpr {
            visibility: Scope::PubUnder(module_path),
            variant: VisibilityExprVariant::Protected,
        }
    }

    pub fn visibility(&self) -> Scope {
        self.visibility
    }

    pub fn parse_from_token_stream(
        db: &dyn VfsDb,
        module_path: ModulePath,
        token_stream: &mut TokenStream<'_>,
    ) -> VisibilityExprResult<Self> {
        Ok(
            if let Some(pub_token) = token_stream.try_parse_optional::<PubToken>()? {
                if let Some(lpar) = token_stream.try_parse_optional::<LeftParenthesisToken>()? {
                    let path_name_token: PathNameToken = token_stream
                        .parse_expected(OriginalVisibilityExprError::ExpectedCrateOrSuper)?;
                    match path_name_token {
                        PathNameToken::Ident(_) => todo!(),
                        PathNameToken::CrateRoot(_) => todo!(),
                        PathNameToken::SelfMod(_) => todo!(),
                        PathNameToken::Super(super_token) => VisibilityExpr {
                            visibility: Scope::PubUnder(module_path.parent(db).ok_or(
                                OriginalVisibilityExprError::NoSuperForRoot(
                                    super_token.token_idx(),
                                ),
                            )?),
                            variant: VisibilityExprVariant::PubUnder {
                                pub_token,
                                lpar,
                                visibility: VisibilityScopeExpr::Super(super_token),
                                rpar: token_stream.parse_expected(
                                    OriginalVisibilityExprError::ExpectedRightParenthesis,
                                )?,
                            },
                        },
                    }
                    // Err(OriginalVisibilityExprError::ExpectedCrateOrSuper(state))?
                } else {
                    VisibilityExpr {
                        visibility: Scope::Pub,
                        variant: VisibilityExprVariant::Pub { pub_token },
                    }
                }
            } else {
                Self::new_protected(module_path)
            },
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum VisibilityExprVariant {
    Protected,
    Pub {
        pub_token: PubToken,
    },
    PubUnder {
        pub_token: PubToken,
        lpar: LeftParenthesisToken,
        visibility: VisibilityScopeExpr,
        rpar: RightParenthesisToken,
    },
}

/// it's guaranteed that the visibility is appropriate
#[derive(Debug, PartialEq, Eq)]
pub enum VisibilityScopeExpr {
    Crate(CrateToken),
    Super(SuperToken),
}
