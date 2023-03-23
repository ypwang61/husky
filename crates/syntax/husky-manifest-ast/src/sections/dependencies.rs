use husky_toml_ast::TomlSection;
use vec_like::VecMap;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ManifestDependenciesSectionAst {
    section_idx: TomlSectionIdx,
    dependencies: VecMap<ManifestDependencyAst>,
}

impl ManifestDependenciesSectionAst {
    pub fn dependencies(&self) -> &[ManifestDependencyAst] {
        self.dependencies.as_ref()
    }
}

impl TransformFromTomlAst<ManifestAstTransformContext> for ManifestDependenciesSectionAst {
    type Ast = TomlSection;

    fn transform_from<'a, 'b>(
        mut transformer: TomlTransformer<'a, 'b, ManifestAstTransformContext, Self::Ast>,
    ) -> ManifestAstResult<Self> {
        Ok(Self {
            section_idx: transformer.section_idx(),
            dependencies: transformer.transform_all_entries_into_vec_map(),
        })
    }
}

impl TransformFromTomlParentKeyed<ManifestAstTransformContext> for ManifestDependenciesSectionAst {
    fn key(menu: &<ManifestAstTransformContext as TomlDeserializeContext>::Menu) -> Word {
        menu.dependencies_word()
    }
}
// impl<'a> ManifestAstTransformer<'a, TomlTable> {
//     pub(crate) fn parse_dependencies_section(
//         &mut self,
//         db: &dyn ManifestAstDb,
//         errors: &mut Vec<ManifestAstError>,
//     ) -> ManifestAstResult<Option<ManifestDependenciesSectionAst>> {
//         let Some(normal_section_parser)= self.normal_section_parser(
//             self.menu().dependencies_word(),
//             errors,
//         )? else {
//             return Ok(None)
//         };
//         Ok(Some(
//             normal_section_parser.parse_into_dependencies_section(errors),
//         ))
//     }
// }

// impl<'a> ManifestAstTransformer<'a, TomlSection> {
//     fn parse_into_dependencies_section(
//         mut self,
//         errors: &mut Vec<ManifestAstError>,
//     ) -> ManifestDependenciesSectionAst {
//         //      Ok(Some(ManifestDependenciesSectionAst {
//         //     idx,
//         //     dependencies: dependencies_section_ast
//         //         .entries()
//         //         .iter()
//         //         .filter_map(|entry| {
//         //             self.entry_parser()
//         //             ManifestDependencyAst::from_toml_section_entry(db, entry, self.expr_, errors)
//         //         })
//         //         .collect(), // ad hoc
//         // }))
//         todo!()
//     }
// }