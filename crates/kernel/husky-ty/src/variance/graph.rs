use super::*;
use vec_like::VecSet;

pub(super) struct VarianceGraph<'a> {
    ids: VecSet<VarianceId>,
    nodes: Vec<VarianceGraphNode<'a>>,
    original_len: usize,
}

impl<'a> Graph for VarianceGraph<'a> {
    type Value = Variance;

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn dependencies(&self, idx: usize) -> &[usize] {
        &self.nodes[idx].dependencies
    }

    fn value_mut(&mut self, idx: usize) -> &mut Self::Value {
        todo!()
    }

    fn eval(&self, idx: usize) -> Self::Value {
        todo!()
    }
}

impl<'a> VarianceGraph<'a> {
    pub(super) fn new(db: &'a dyn TypeDb, path: EntityPath) -> VarianceResult<Self> {
        let Ok(entity_variance_reprs) = entity_variance_reprs(db, path)
            else {
                todo!()
            };
        let mut ids: VecSet<VarianceId> = Default::default();
        let nodes = entity_variance_reprs
            .iter()
            .map(|repr| VarianceGraphNode::new(&mut ids, repr))
            .collect();
        let mut this = Self {
            ids,
            nodes,
            original_len: entity_variance_reprs.len(),
        };
        this.init();
        Ok(this)
    }

    fn init(&mut self) {
        while self.ids.len() > self.nodes.len() {
            todo!()
        }
    }

    pub(crate) fn finish(&self) -> Vec<Variance> {
        self.nodes[0..self.original_len]
            .iter()
            .map(|node| node.value)
            .collect()
    }
}

pub(super) struct VarianceGraphNode<'a> {
    repr: &'a VarianceRepr,
    value: Variance,
    dependencies: Vec<usize>,
}

impl<'a> VarianceGraphNode<'a> {
    pub(super) fn new(ids: &mut VecSet<VarianceId>, repr: &'a VarianceRepr) -> Self {
        Self {
            repr,
            value: repr.base(),
            dependencies: repr.dependencies().iter().map(|_| todo!()).collect(),
        }
    }
}