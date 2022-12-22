use crate::*;
use husky_entity_kind::EntityKind;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EntityNode {
    entity_path: EntityPath,
    accessibility: Accessibility,
    card: EntityKind,
}

impl std::ops::Deref for EntityTree {
    type Target = EntityNode;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl EntityNode {
    pub(crate) fn new(
        entity_path: EntityPath,
        accessibility: Accessibility,
        card: EntityKind,
    ) -> Self {
        Self {
            entity_path,
            accessibility,
            card,
        }
    }

    pub(crate) fn entity_path(&self) -> EntityPath {
        self.entity_path
    }

    pub(crate) fn accessibility(&self) -> Accessibility {
        self.accessibility
    }

    pub(crate) fn card(&self) -> EntityKind {
        self.card
    }
}

// #[salsa::tracked(jar = EntityTreeJar, return_ref)]
// pub(crate) fn entity_node(
//     db: &dyn EntityTreeDb,
//     entity_path: EntityPath,
// ) -> EntityTreeResult<EntityNode> {
//     match entity_path.data(db) {
//         EntityPathData::Module(_) => todo!(),
//         EntityPathData::Associated { parent, ident } => todo!(),
//     }
//     // Ok(
//     //     if let Some(parent_module) = parent_module(db, entity_path).as_ref()? {
//     //         let entity_tree_page = db.entity_tree_sheet(*parent_module).as_ref()?;
//     //         if let Some(tree) = entity_tree_page.get(entity_path) {
//     //             tree.node.clone()
//     //         } else {
//     //             todo!()
//     //         }
//     //     } else {
//     //         let _entity_tree_page = db.entity_tree_sheet(entity_path).as_ref()?;
//     //         EntityNode {
//     //             entity_path,
//     //             accessibility: Accessibility::Public,
//     //             card: EntityCard::Module,
//     //         }
//     //     },
//     // )
// }

// #[salsa::tracked(jar = EntityTreeJar, return_ref)]
// pub(crate) fn entity_kind(
//     db: &dyn EntityTreeDb,
//     entity_path: EntityPath,
// ) -> EntityTreeResult<EntityCard> {
//     Ok(entity_node(db, entity_path).as_ref()?.card())
// }

// #[salsa::tracked(jar = EntityTreeJar, return_ref)]
// pub(crate) fn entity_accessibility(
//     db: &dyn EntityTreeDb,
//     entity_path: EntityPath,
// ) -> EntityTreeResult<Accessibility> {
//     Ok(entity_node(db, entity_path).as_ref()?.accessibility())
// }

// #[salsa::tracked(jar = EntityTreeJar, return_ref)]
// pub(crate) fn parent_module(
//     db: &dyn EntityTreeDb,
//     entity_path: EntityPath,
// ) -> EntityTreeResult<Option<ModulePath>> {
//     Ok(match entity_path.data(db) {
//         EntityPathData::Module(_) => todo!(),
//         EntityPathData::Associated { parent, ident: _ } => {
//             todo!()
//             // match entity_kind(db, parent).as_ref()? {
//             //     EntityCard::Module => Some(parent),
//             //     _ => *parent_module(db, parent).as_ref()?,
//             // }
//         }
//     })
// }