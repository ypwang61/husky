use super::*;

/// extract from library for safer type wrapping and more efficient lookup
pub(super) fn generate_map<Linkage: IsLinkage>(
    target_crate: CratePath,
    library: &BootLibraryStorage,
    db: &dyn HirDepsDb,
) -> HashMap<LinkagePath, (HirLinkageDeps, Linkage)> {
    todo!()
}