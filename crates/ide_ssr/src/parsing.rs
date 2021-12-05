//! This file contains code for parsing SSR rules, which look something like `foo($a) ==>> bar($b)`.
//! We first split everything before and after the separator `==>>`. Next, both the search pattern
//! and the replacement template get tokenized by the Rust tokenizer. Tokens are then searched for
//! placeholders, which start with `$`. For replacement templates, this is the final form. For
//! search patterns, we go further and parse the pattern as each kind of thing that we can match.
//! e.g. expressions, type references etc.

use crate::errors::bail;
use crate::{SsrError, SsrPattern, SsrRule};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{fmt::Display, str::FromStr};
use syntax::{SmolStr, SyntaxKind, SyntaxNode};

#[derive(Debug)]
pub(crate) struct ParsedRule {
    pub(crate) placeholders_by_stand_in: FxHashMap<SmolStr, Placeholder>,
    pub(crate) pattern: SyntaxNode,
    pub(crate) template: Option<SyntaxNode>,
}

#[derive(Debug)]
pub(crate) struct RawPattern {
    tokens: Vec<PatternElement>,
}

// Part of a search or replace pattern.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum PatternElement {
    Token(Token),
    Placeholder(Placeholder),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Placeholder {
    /// The name of this placeholder. e.g. for "$a", this would be "a"
    pub(crate) ident: Var,
    /// A unique name used in place of this placeholder when we parse the pattern as Rust code.
    stand_in_name: String,
    pub(crate) constraints: Vec<Constraint>,
}

/// Represents a `$var` in an SSR query.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Var(pub(crate) String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Constraint {
    Kind(NodeKind),
    Not(Box<Constraint>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum NodeKind {
    Literal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Token {
    kind: SyntaxKind,
    pub(crate) text: SmolStr,
}

impl ParsedRule {
    fn new(
        pattern: &RawPattern,
        template: Option<&RawPattern>,
    ) -> Result<Vec<ParsedRule>, SsrError> {
        todo!()
    }
}

struct RuleBuilder {
    placeholders_by_stand_in: FxHashMap<SmolStr, Placeholder>,
    rules: Vec<ParsedRule>,
}

impl RuleBuilder {
    fn try_add(
        &mut self,
        pattern: Result<SyntaxNode, ()>,
        template: Option<Result<SyntaxNode, ()>>,
    ) {
        todo!()
    }

    fn build(mut self) -> Result<Vec<ParsedRule>, SsrError> {
        if self.rules.is_empty() {
            bail!("Not a valid Rust expression, type, item, path or pattern");
        }
        // If any rules contain paths, then we reject any rules that don't contain paths. Allowing a
        // mix leads to strange semantics, since the path-based rules only match things where the
        // path refers to semantically the same thing, whereas the non-path-based rules could match
        // anything. Specifically, if we have a rule like `foo ==>> bar` we only want to match the
        // `foo` that is in the current scope, not any `foo`. However "foo" can be parsed as a
        // pattern (IDENT_PAT -> NAME -> IDENT). Allowing such a rule through would result in
        // renaming everything called `foo` to `bar`. It'd also be slow, since without a path, we'd
        // have to use the slow-scan search mechanism.
        if self.rules.iter().any(|rule| contains_path(&rule.pattern)) {
            let old_len = self.rules.len();
            self.rules.retain(|rule| contains_path(&rule.pattern));
            if self.rules.len() < old_len {
                cov_mark::hit!(pattern_is_a_single_segment_path);
            }
        }
        Ok(self.rules)
    }
}

/// Returns whether there are any paths in `node`.
fn contains_path(node: &SyntaxNode) -> bool {
    todo!()
}

impl FromStr for SsrRule {
    type Err = SsrError;

    fn from_str(query: &str) -> Result<SsrRule, SsrError> {
        let mut it = query.split("==>>");
        let pattern = it.next().expect("at least empty string").trim();
        let template = it
            .next()
            .ok_or_else(|| SsrError("Cannot find delimiter `==>>`".into()))?
            .trim()
            .to_string();
        if it.next().is_some() {
            return Err(SsrError("More than one delimiter found".into()));
        }
        let raw_pattern = pattern.parse()?;
        let raw_template = template.parse()?;
        let parsed_rules = ParsedRule::new(&raw_pattern, Some(&raw_template))?;
        let rule = SsrRule {
            pattern: raw_pattern,
            template: raw_template,
            parsed_rules,
        };
        validate_rule(&rule)?;
        Ok(rule)
    }
}

impl FromStr for RawPattern {
    type Err = SsrError;

    fn from_str(pattern_str: &str) -> Result<RawPattern, SsrError> {
        Ok(RawPattern {
            tokens: parse_pattern(pattern_str)?,
        })
    }
}

impl RawPattern {
    /// Returns this search pattern as Rust source code that we can feed to the Rust parser.
    fn as_rust_code(&self) -> String {
        let mut res = String::new();
        for t in &self.tokens {
            res.push_str(match t {
                PatternElement::Token(token) => token.text.as_str(),
                PatternElement::Placeholder(placeholder) => placeholder.stand_in_name.as_str(),
            });
        }
        res
    }

    pub(crate) fn placeholders_by_stand_in(&self) -> FxHashMap<SmolStr, Placeholder> {
        let mut res = FxHashMap::default();
        for t in &self.tokens {
            if let PatternElement::Placeholder(placeholder) = t {
                res.insert(
                    SmolStr::new(placeholder.stand_in_name.clone()),
                    placeholder.clone(),
                );
            }
        }
        res
    }
}

impl FromStr for SsrPattern {
    type Err = SsrError;

    fn from_str(pattern_str: &str) -> Result<SsrPattern, SsrError> {
        let raw_pattern = pattern_str.parse()?;
        let parsed_rules = ParsedRule::new(&raw_pattern, None)?;
        Ok(SsrPattern { parsed_rules })
    }
}

/// Returns `pattern_str`, parsed as a search or replace pattern. If `remove_whitespace` is true,
/// then any whitespace tokens will be removed, which we do for the search pattern, but not for the
/// replace pattern.
fn parse_pattern(pattern_str: &str) -> Result<Vec<PatternElement>, SsrError> {
    todo!()
}

/// Checks for errors in a rule. e.g. the replace pattern referencing placeholders that the search
/// pattern didn't define.
fn validate_rule(rule: &SsrRule) -> Result<(), SsrError> {
    let mut defined_placeholders = FxHashSet::default();
    for p in &rule.pattern.tokens {
        if let PatternElement::Placeholder(placeholder) = p {
            defined_placeholders.insert(&placeholder.ident);
        }
    }
    let mut undefined = Vec::new();
    for p in &rule.template.tokens {
        if let PatternElement::Placeholder(placeholder) = p {
            if !defined_placeholders.contains(&placeholder.ident) {
                undefined.push(placeholder.ident.to_string());
            }
            if !placeholder.constraints.is_empty() {
                bail!("Replacement placeholders cannot have constraints");
            }
        }
    }
    if !undefined.is_empty() {
        bail!(
            "Replacement contains undefined placeholders: {}",
            undefined.join(", ")
        );
    }
    Ok(())
}

fn tokenize(source: &str) -> Result<Vec<Token>, SsrError> {
    todo!()
}

fn parse_placeholder(tokens: &mut std::vec::IntoIter<Token>) -> Result<Placeholder, SsrError> {
    todo!()
}

fn parse_constraint(tokens: &mut std::vec::IntoIter<Token>) -> Result<Constraint, SsrError> {
    todo!()
}

fn expect_token(tokens: &mut std::vec::IntoIter<Token>, expected: &str) -> Result<(), SsrError> {
    if let Some(t) = tokens.next() {
        if t.text == expected {
            return Ok(());
        }
        bail!("Expected {} found {}", expected, t.text);
    }
    bail!("Expected {} found end of stream", expected);
}

impl NodeKind {
    fn from(name: &SmolStr) -> Result<NodeKind, SsrError> {
        Ok(match name.as_str() {
            "literal" => NodeKind::Literal,
            _ => bail!("Unknown node kind '{}'", name),
        })
    }
}

impl Placeholder {
    fn new(name: SmolStr, constraints: Vec<Constraint>) -> Self {
        Self {
            stand_in_name: format!("__placeholder_{}", name),
            constraints,
            ident: Var(name.to_string()),
        }
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.0)
    }
}
