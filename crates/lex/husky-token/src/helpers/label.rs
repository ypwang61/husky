use husky_coword::Label;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenDb)]
pub struct LifetimeLabelToken {
    label: Label,
    token_idx: TokenIdx,
}

impl LifetimeLabelToken {
    pub fn label(&self) -> Label {
        self.label
    }

    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for LifetimeLabelToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                Token::Label(label) if label.is_valid_lifetime_label() => {
                    Ok(Some(LifetimeLabelToken { label, token_idx }))
                }
                Token::Error(error) => Err(error),
                Token::Label(_)
                | Token::Ident(_)
                | Token::Punctuation(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

#[test]
fn lifetime_label_token_works() {
    // todo
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenDb)]
pub struct BindingLabelToken {
    label: Label,
    token_idx: TokenIdx,
}

impl BindingLabelToken {
    pub fn label(&self) -> Label {
        self.label
    }

    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for BindingLabelToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                Token::Label(label) if label.is_valid_binding_label() => {
                    Ok(Some(BindingLabelToken { label, token_idx }))
                }
                Token::Error(error) => Err(error),
                Token::Label(_)
                | Token::Ident(_)
                | Token::Punctuation(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

#[test]
fn aux_ident_token_works() {
    // todo
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenDb)]
pub struct BlockLabelToken {
    label: Label,
    token_idx: TokenIdx,
}

impl BlockLabelToken {
    pub fn ident(&self) -> Label {
        self.label
    }

    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for BlockLabelToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                Token::Label(label) => Ok(Some(BlockLabelToken { label, token_idx })),
                Token::Error(error) => Err(error),
                Token::Ident(_)
                | Token::Punctuation(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

#[test]
fn lifetime_ident_token_works() {
    // todo
}