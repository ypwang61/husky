use super::*;
use vec_like::VecSet;

// `Default` is derived because we never inherited hollow terms
#[derive(Debug, Default, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct HollowTerms {
    entries: Vec<HollowTermEntry>,
    first_unresolved_term_idx: usize,
}

impl HollowTerms {
    // for ide
    pub fn errors(&self) -> impl Iterator<Item = (HoleSource, &OriginalHollowTermResolveError)> {
        self.entries.iter().filter_map(|entry| match entry {
            HollowTermEntry {
                data: HollowTermData::Hole { hole_source, .. },
                resolve_progress:
                    HollowTermResolveProgressBuf::Err(HollowTermResolveError::Original(e)),
            } => Some((*hole_source, e)),
            _ => None,
        })
    }

    // alloc something that's actually different
    #[inline(always)]
    pub(crate) fn alloc_new(&mut self, data: HollowTermData) -> HollowTerm {
        let idx = self.entries.len();
        self.entries.push(HollowTermEntry {
            data,
            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
        });
        HollowTerm(idx.try_into().expect("within range"))
    }

    pub(crate) fn resolve_progress(&self, hollow_term: HollowTerm) -> HollowTermResolveProgress {
        self.entry(hollow_term).resolve_progress.share()
    }

    pub(crate) fn data(&self, hollow_term: HollowTerm) -> &HollowTermData {
        &self.entry(hollow_term).data
    }

    fn entry(&self, hollow_term: HollowTerm) -> &HollowTermEntry {
        &self.entries[hollow_term.idx()]
    }

    pub(crate) fn fill_hole(&mut self, db: &dyn FluffyTermDb, hole: Hole, term: FluffyTerm) {
        let mut hollow_term_entry = &mut self.entries[hole.idx()];
        match hollow_term_entry.data {
            HollowTermData::Hole { ref mut fill, .. } => *fill = Some(term),
            HollowTermData::Hole { fill: Some(_), .. } => unreachable!(),
            _ => unreachable!(),
        }
        // update progress if term is resolved
        match term.nested() {
            NestedFluffyTerm::Ethereal(term) => {
                hollow_term_entry.resolve_progress =
                    HollowTermResolveProgressBuf::ResolvedEthereal(term)
            }
            NestedFluffyTerm::Solid(term) => {
                hollow_term_entry.resolve_progress =
                    HollowTermResolveProgressBuf::ResolvedSolid(term)
            }
            NestedFluffyTerm::Hollow(_) => (),
        }
        self.update_entries(db)
    }

    fn update_entries(&mut self, db: &dyn FluffyTermDb) {
        let first_unresolved_idx = self.get_first_unresolved_term_idx();
        for idx in first_unresolved_idx..self.entries.len() {
            self.try_update_entry(db, idx)
        }
    }

    // lazy update
    fn get_first_unresolved_term_idx(&mut self) -> usize {
        let mut idx = self.first_unresolved_term_idx;
        // lazy update
        while idx < self.entries.len() {
            match self.entries[idx].is_resolved() {
                true => idx += 1,
                false => break,
            }
        }
        self.first_unresolved_term_idx = idx;
        idx
    }

    fn try_update_entry(&mut self, db: &dyn FluffyTermDb, idx: usize) {
        if self.entries[idx].is_resolved() {
            return;
        }
        match self.entries[idx].data {
            HollowTermData::TypeOntology {
                path,
                refined_path,
                ref arguments,
            } => {
                let mut has_solid = false;
                for argument in arguments {
                    match argument.resolve_progress(self) {
                        HollowTermResolveProgress::Unresolved => return,
                        HollowTermResolveProgress::ResolvedEthereal(_) => todo!(),
                        HollowTermResolveProgress::ResolvedSolid(_) => todo!(),
                        HollowTermResolveProgress::Err => todo!(),
                    }
                }
                // let mut resolved_arguments = smallvec![];
                // resolved_arguments.reserve(arguments.len());
            }
            HollowTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
            } => todo!(),
            HollowTermData::Hole { fill, .. } => match fill {
                Some(fill) => match fill.resolve_progress(self) {
                    HollowTermResolveProgress::Unresolved => return,
                    HollowTermResolveProgress::ResolvedEthereal(_) => todo!(),
                    HollowTermResolveProgress::ResolvedSolid(_) => todo!(),
                    HollowTermResolveProgress::Err => todo!(),
                },
                None => (),
            },
            HollowTermData::Ritchie {
                ritchie_kind,
                ref parameter_contracted_tys,
                return_ty,
            } => todo!(),
            HollowTermData::PlaceTypeOntology {
                place,
                path,
                refined_path,
                ref arguments,
            } => todo!(),
            HollowTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => match hole.term().resolve_progress(self) {
                HollowTermResolveProgress::Unresolved => return,
                HollowTermResolveProgress::ResolvedEthereal(_) => todo!(),
                HollowTermResolveProgress::ResolvedSolid(_) => todo!(),
                HollowTermResolveProgress::Err => todo!(),
            },
        }
    }
}

impl FluffyTerm {
    fn resolve_progress(self, hollow_terms: &HollowTerms) -> HollowTermResolveProgress {
        match self.nested() {
            NestedFluffyTerm::Ethereal(term) => HollowTermResolveProgress::ResolvedEthereal(term),
            NestedFluffyTerm::Solid(term) => HollowTermResolveProgress::ResolvedSolid(term),
            NestedFluffyTerm::Hollow(term) => term.resolve_progress(hollow_terms),
        }
    }
}

impl HollowTerm {
    fn resolve_progress(self, hollow_terms: &HollowTerms) -> HollowTermResolveProgress {
        hollow_terms.entries[self.idx()].resolve_progress.share()
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct HollowTermEntry {
    data: HollowTermData,
    resolve_progress: HollowTermResolveProgressBuf,
}

impl HollowTermEntry {
    pub(super) fn force_resolve_term(&mut self) -> Option<NestedFluffyTerm> {
        todo!()
        // match self.resolve_progress {
        //     Ok(FluffyTerm::EtherealTerm(term)) => Some(term),
        //     Ok(FluffyTerm::Unresolved(_)) => {
        //         self.resolve_progress = Err(OriginalFluffyTermResolveError::UnresolvedTerm.into());
        //         None
        //     }
        //     Ok(FluffyTerm::PlaceType(_)) => todo!(),
        //     Err(_) => None,
        // }
    }

    pub fn data(&self) -> &HollowTermData {
        &self.data
    }

    pub(crate) fn resolve_progress(&self) -> Option<FluffyTerm> {
        todo!()
        // match self.resolve_progress {
        //     Ok(resolve_progress) => Some(resolve_progress),
        //     Err(_) => None,
        // }
    }

    pub(crate) fn is_resolved(&self) -> bool {
        match self.resolve_progress {
            HollowTermResolveProgressBuf::Unresolved => false,
            _ => true,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub enum HollowTermResolveProgress {
    Unresolved,
    ResolvedEthereal(EtherealTerm),
    ResolvedSolid(SolidTerm),
    Err,
}

impl HollowTermResolveProgressBuf {
    fn share(&self) -> HollowTermResolveProgress {
        match self {
            HollowTermResolveProgressBuf::Unresolved => HollowTermResolveProgress::Unresolved,
            HollowTermResolveProgressBuf::ResolvedEthereal(term) => {
                HollowTermResolveProgress::ResolvedEthereal(*term)
            }
            HollowTermResolveProgressBuf::ResolvedSolid(term) => {
                HollowTermResolveProgress::ResolvedSolid(*term)
            }
            HollowTermResolveProgressBuf::Err(_) => HollowTermResolveProgress::Err,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub enum HollowTermResolveProgressBuf {
    Unresolved,
    ResolvedEthereal(EtherealTerm),
    ResolvedSolid(SolidTerm),
    Err(HollowTermResolveError),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct HollowTerm(u32);

impl HollowTerm {
    #[inline(always)]
    pub(crate) fn idx(self) -> usize {
        self.0 as usize
    }
}