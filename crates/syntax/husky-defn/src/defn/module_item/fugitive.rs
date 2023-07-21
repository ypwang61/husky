mod r#fn;
mod gn;
mod type_alias;
mod val;

pub use self::gn::*;
pub use self::r#fn::*;
pub use self::type_alias::*;
pub use self::val::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum FugitiveNodeDefn {
    Fn(FnNodeDefn),
    // Function(FunctionDefn),
    Val(ValNodeDefn),
    Gn(GnNodeDefn),
    // AliasType(TypeAliasDefn)
}

impl FugitiveNodeDefn {
    pub fn node_decl(self, db: &dyn DefnDb) -> FugitiveNodeDecl {
        match self {
            FugitiveNodeDefn::Fn(node_defn) => node_defn.node_decl(db).into(),
            FugitiveNodeDefn::Val(node_defn) => node_defn.node_decl(db).into(),
            FugitiveNodeDefn::Gn(node_defn) => node_defn.node_decl(db).into(),
        }
    }

    pub fn node_path(self, db: &dyn DefnDb) -> FugitiveNodePath {
        todo!()
        // match self {
        //     FugitiveDefn::Fn(defn) => defn.path(db),
        //     FugitiveDefn::Val(defn) => defn.path(db),
        //     FugitiveDefn::Gn(defn) => defn.path(db),
        // }
    }

    pub fn expr_region(self, db: &dyn DefnDb) -> ExprRegion {
        match self {
            FugitiveNodeDefn::Fn(defn) => defn.expr_region(db),
            FugitiveNodeDefn::Val(defn) => defn.expr_region(db),
            FugitiveNodeDefn::Gn(defn) => defn.expr_region(db),
        }
    }
}

impl HasNodeDefn for FugitiveNodePath {
    type NodeDefn = FugitiveNodeDefn;

    fn node_defn(self, db: &dyn DefnDb) -> Self::NodeDefn {
        fugitive_node_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn fugitive_node_defn(db: &dyn DefnDb, node_path: FugitiveNodePath) -> FugitiveNodeDefn {
    match node_path.node_decl(db) {
        FugitiveNodeDecl::Fn(node_decl) => FnNodeDefn::new(db, node_path, node_decl).into(),
        FugitiveNodeDecl::Val(node_decl) => ValNodeDefn::new(db, node_path, node_decl).into(),
        FugitiveNodeDecl::Gn(node_decl) => GnNodeDefn::new(db, node_path, node_decl).into(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum FugitiveDefn {
    Fn(FnDefn),
    // Function(FunctionDefn),
    Val(ValDefn),
    Gn(GnDefn),
    // AliasType(TypeAliasDefn)
}

impl FugitiveDefn {
    pub fn decl(self, db: &dyn DefnDb) -> FugitiveDecl {
        match self {
            FugitiveDefn::Fn(defn) => defn.decl(db).into(),
            FugitiveDefn::Val(defn) => defn.decl(db).into(),
            FugitiveDefn::Gn(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, db: &dyn DefnDb) -> FugitivePath {
        todo!()
        // match self {
        //     FugitiveDefn::Fn(defn) => defn.path(db),
        //     FugitiveDefn::Val(defn) => defn.path(db),
        //     FugitiveDefn::Gn(defn) => defn.path(db),
        // }
    }
    pub fn expr_region(self, db: &dyn DefnDb) -> ExprRegion {
        match self {
            FugitiveDefn::Fn(defn) => defn.expr_region(db),
            FugitiveDefn::Val(defn) => defn.expr_region(db),
            FugitiveDefn::Gn(defn) => defn.expr_region(db),
        }
    }
}

impl HasDefn for FugitivePath {
    type Defn = FugitiveDefn;

    fn defn(self, db: &dyn DefnDb) -> DefnResult<Self::Defn> {
        fugitive_defn(db, self)
    }
}

#[salsa::tracked(jar= SynDefnJar)]
pub(crate) fn fugitive_defn(db: &dyn DefnDb, path: FugitivePath) -> DefnResult<FugitiveDefn> {
    Ok(match path.decl(db)? {
        FugitiveDecl::Fn(decl) => FnDefn::new(db, path, decl).into(),
        FugitiveDecl::Val(decl) => ValDefn::new(db, path, decl).into(),
        FugitiveDecl::Gn(decl) => GnDefn::new(db, path, decl).into(),
    })
}
