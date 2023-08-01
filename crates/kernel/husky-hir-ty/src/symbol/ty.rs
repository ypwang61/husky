use husky_term_prelude::Variance;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirTypeSymbol {
    Type {
        attrs: HirSymbolAttrs,
        variance: Option<Variance>,
        disambiguator: u8,
    },
    SelfType,
}

impl HirTypeSymbol {
    pub(crate) fn from_ethereal(symbol: EtherealTermSymbol, db: &dyn HirTypeDb) -> Self {
        match symbol.index(db).inner() {
            EtherealTermSymbolIndexInner::Lifetime {
                attrs,
                variance,
                disambiguator,
            } => todo!(),
            EtherealTermSymbolIndexInner::Type {
                attrs,
                variance,
                disambiguator,
            } => HirTypeSymbol::Type {
                attrs: HirSymbolAttrs::from_ethereal(attrs).expect("some"),
                variance,
                disambiguator,
            },
            EtherealTermSymbolIndexInner::Prop { disambiguator } => {
                todo!()
            }
            EtherealTermSymbolIndexInner::ConstPathLeading {
                attrs,
                disambiguator,
                ty_path,
            } => todo!(),
            EtherealTermSymbolIndexInner::ConstOther {
                attrs,
                disambiguator,
            } => todo!(),
            EtherealTermSymbolIndexInner::EphemPathLeading {
                disambiguator,
                ty_path,
            } => todo!(),
            EtherealTermSymbolIndexInner::EphemOther { disambiguator } => {
                todo!()
            }
            EtherealTermSymbolIndexInner::SelfType => HirTypeSymbol::SelfType,
            EtherealTermSymbolIndexInner::SelfValue => todo!(),
        }
    }
}