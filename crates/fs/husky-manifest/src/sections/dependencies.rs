use super::*;
use vec_like::VecSet;

#[salsa::tracked(db = ManifestDb, jar = ManifestJar)]
pub struct PackageDependenciesSection {
    #[return_ref]
    pub data: Vec<PackageDependency>,
}

pub(crate) fn package_dependencies(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> ManifestResultRef<&[PackageDependency]> {
    // todo!(): check for cycles!
    package_dependencies_unchecked(db, package_path)
}

fn package_dependencies_unchecked(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> ManifestResultRef<&[PackageDependency]> {
    package_dependencies_unchecked_aux(db, package_path)
        .as_ref()
        .map(|s| s.data(db) as &[_])
}

#[salsa::tracked(jar = ManifestJar, return_ref)]
pub(crate) fn package_dependencies_unchecked_aux(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> ManifestResult<PackageDependenciesSection> {
    Ok(package_path.package_manifest(db)?.dependencies(db))
}

fn cyclic_dependent_package_paths(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> ManifestResultRef<&[PackagePath]> {
    cyclic_dependent_package_paths_aux(db, package_path)
        .as_ref()
        .map(|s| s as &[_])
}

#[salsa::tracked(jar = ManifestJar, return_ref)]
pub(crate) fn cyclic_dependent_package_paths_aux(
    _db: &::salsa::Db,
    _package_path: PackagePath,
) -> ManifestResult<VecSet<PackagePath>> {
    todo!()
}

/// includes package_path itself
pub(crate) fn full_dependent_package_paths(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> ManifestResultRef<&[PackagePath]> {
    full_dependent_package_paths_aux(db, package_path)
        .as_ref()
        .map(|s| s as &[_])
}

#[salsa::tracked(jar = ManifestJar, return_ref)]
pub(crate) fn full_dependent_package_paths_aux(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> ManifestResult<VecSet<PackagePath>> {
    let mut package_paths: VecSet<PackagePath> = VecSet::new_one_elem_set(package_path);
    let mut first_unsearched = 0usize;
    while first_unsearched < package_paths.len() {
        let first_unsearched = std::mem::replace(&mut first_unsearched, package_paths.len());
        for i in first_unsearched..package_paths.len() {
            package_paths.extend(
                package_paths[i]
                    .dependencies(db)?
                    .iter()
                    .map(|dep| dep.package_path()),
            );
        }
    }
    Ok(package_paths)
}