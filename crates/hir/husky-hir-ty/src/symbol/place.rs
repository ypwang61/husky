use super::*;
use husky_term_prelude::Variance;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirPlaceSymbol {
    pub(crate) attrs: HirSymbolAttrs,
    pub(crate) variance: Option<Variance>,
    pub(crate) disambiguator: u8,
}

impl HirPlaceSymbol {
    pub fn attrs(&self) -> &HirSymbolAttrs {
        &self.attrs
    }

    pub fn variance(&self) -> Option<Variance> {
        self.variance
    }

    pub fn disambiguator(&self) -> u8 {
        self.disambiguator
    }
}