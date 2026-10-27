//! Hello-world snippet for `ProgrammingLanguageName::Migol`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Migol>
//! - <https://esolangs.org/wiki/Migol_11>
//! - <https://esolangs.org/wiki/Migol> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Imperative;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Migol => "Migol",
    versions: [Migol => "Migol"],
    extensions: [],
}

const MIGOL: &str = r"'H>,'e>,'l>,'l>,'o>,',>,' >,'w>,'o>,'r>,'l>,'d>,'!>,10>";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Migol,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Migol,
            syntax: MIGOL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MIGOL,
                    [(
                        TokenKind::Plain,
                        r"'H>,'e>,'l>,'l>,'o>,',>,' >,'w>,'o>,'r>,'l>,'d>,'!>,10>",
                    )],
                )
            },
        }],
    };
