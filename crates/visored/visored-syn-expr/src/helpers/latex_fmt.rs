use super::*;
#[cfg(test)]
use crate::test_helpers::builder::VdSynExprTestBuilder;
use crate::{
    clause::{VdSynClauseArenaRef, VdSynClauseData, VdSynClauseIdx},
    expr::{VdSynExprArenaRef, VdSynExprData, VdSynExprIdx},
    phrase::{
        noun::VdSynNounPhraseData, VdSynPhrase, VdSynPhraseArenaRef, VdSynPhraseData,
        VdSynPhraseIdx,
    },
    sentence::{VdSynSentenceArenaRef, VdSynSentenceData, VdSynSentenceIdx, VdSynSentenceIdxRange},
};
use either::*;
use visored_opr::opr::binary::VdBinaryOpr;
use visored_zfs_ty::{menu::vd_zfs_ty_menu, term::literal::VdZfsLiteralData};

pub struct VdSynExprLaTeXFormatter<'a> {
    db: &'a ::salsa::Db,
    expr_arena: VdSynExprArenaRef<'a>,
    phrase_arena: VdSynPhraseArenaRef<'a>,
    clause_arena: VdSynClauseArenaRef<'a>,
    sentence_arena: VdSynSentenceArenaRef<'a>,
    result: String,
}

impl<'a> VdSynExprLaTeXFormatter<'a> {
    pub fn new(
        db: &'a ::salsa::Db,
        expr_arena: VdSynExprArenaRef<'a>,
        phrase_arena: VdSynPhraseArenaRef<'a>,
        clause_arena: VdSynClauseArenaRef<'a>,
        sentence_arena: VdSynSentenceArenaRef<'a>,
    ) -> Self {
        Self {
            db,
            expr_arena,
            phrase_arena,
            clause_arena,
            sentence_arena,
            result: Default::default(),
        }
    }

    pub fn fmt_sentences(&mut self, sentences: VdSynSentenceIdxRange) {
        for sentence_idx in sentences {
            self.fmt_sentence(sentence_idx);
            self.result.push_str("\n\n");
        }
    }

    pub fn fmt_sentence(&mut self, sentence_idx: VdSynSentenceIdx) {
        match self.sentence_arena[sentence_idx] {
            VdSynSentenceData::Clauses(clauses) => {
                for (index, clause_idx) in clauses.into_iter().enumerate() {
                    self.fmt_clause(clause_idx);
                    if index < clauses.len() - 1 {
                        self.result.push_str(", ");
                    } else {
                        self.result.push_str(". ");
                    }
                }
            }
        }
    }

    pub fn fmt_clause(&mut self, clause_idx: VdSynClauseIdx) {
        match self.clause_arena[clause_idx] {
            VdSynClauseData::Verb => todo!(),
        }
    }

    pub fn fmt_phrase(&mut self, phrase_idx: VdSynPhraseIdx) {
        match self.phrase_arena[phrase_idx] {
            VdSynPhraseData::Noun(ref vd_syn_noun_phrase_data) => todo!(),
        }
    }

    fn fmt_noun_phrase(&mut self, noun_phrase: &VdSynNounPhraseData) {
        // Implement noun phrase formatting
        // This is a placeholder implementation
        // self.result.push_str("\\textbf{");
        // self.fmt_expr(noun_phrase.head);
        // self.result.push('}');
        todo!()
    }

    pub fn fmt_expr(&mut self, expr_idx: VdSynExprIdx) {
        let db = self.db;
        match self.expr_arena[expr_idx] {
            VdSynExprData::Literal { literal } => match literal.data(db) {
                VdZfsLiteralData::NaturalNumber(s) => {
                    if self
                        .result
                        .chars()
                        .last()
                        .map_or(false, |c| c.is_alphanumeric())
                    {
                        self.result.push(' ');
                    }
                    self.result.push_str(s);
                }
                VdZfsLiteralData::NegativeInteger(_) => todo!(),
                VdZfsLiteralData::FiniteDecimalRepresentation(_) => {
                    todo!()
                }
                VdZfsLiteralData::SpecialConstant(vd_zfs_special_constant) => todo!(),
            },
            VdSynExprData::Notation => todo!(),
            VdSynExprData::Binary {
                lopd, opr, ropd, ..
            } => {
                self.fmt_expr(lopd);
                match opr {
                    Left(opr) => self.result.push_str(opr.latex_code()),
                    Either::Right(opr) => self.fmt_expr(opr),
                }
                self.fmt_expr(ropd);
            }
            VdSynExprData::Prefix { opr, opd } => todo!(),
            VdSynExprData::Suffix { opd, opr } => todo!(),
            VdSynExprData::Attach { .. } => todo!(),
            VdSynExprData::UniadicChain => todo!(),
            VdSynExprData::VariadicChain => todo!(),
            VdSynExprData::UniadicArray => todo!(),
            VdSynExprData::VariadicArray => todo!(),
            VdSynExprData::Opr { opr } => todo!(),
            VdSynExprData::Err(ref error) => unreachable!("{error}"),
        }
    }

    pub fn finish(self) -> String {
        self.result
    }
}

#[test]
fn latex_fmt_works() {
    let db = &DB::default();
    let menu = vd_zfs_ty_menu(db);
    let mut builder = VdSynExprTestBuilder::new(db);
    let one = builder.new_expr_checked(
        VdSynExprData::Literal {
            literal: menu.one_literal(),
        },
        "1",
    );
    let one_add_one = builder.new_expr_checked(
        VdSynExprData::Binary {
            lopd: one,
            opr: Either::Left(VdBinaryOpr::Add),
            ropd: one,
        },
        "1+1",
    );
}
