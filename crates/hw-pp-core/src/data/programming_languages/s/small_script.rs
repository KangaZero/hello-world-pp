//! Hello-world snippet for `ProgrammingLanguageName::SmallSCRIPT`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#small_s.c.r.i.p.t.>
//! - <https://esolangs.org/wiki/Small_s.c.r.i.p.t.>

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: SmallSCRIPT => "small s.c.r.i.p.t.",
    versions: [SmallSCRIPT => "small s.c.r.i.p.t."],
    extensions: [],
}

const SMALL_SCRIPT: &str = r" Hello\, world";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::SmallSCRIPT,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SmallSCRIPT,
            syntax: SMALL_SCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(SMALL_SCRIPT, [(TokenKind::Plain, r" Hello\, world")])
            },
        }],
    };
