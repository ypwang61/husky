use crate::*;
use husky_hir_ty::db::HirTypeDb;
use husky_sema_expr::SemaExprDb;

pub trait HirLazyExprDb: salsa::DbWithJar<HirLazyExprJar> + SemaExprDb + HirTypeDb {}

impl<Db> HirLazyExprDb for Db where Db: salsa::DbWithJar<HirLazyExprJar> + SemaExprDb + HirTypeDb {}

#[salsa::jar(db = HirLazyExprDb)]
pub struct HirLazyExprJar(HirLazyExprRegion);
