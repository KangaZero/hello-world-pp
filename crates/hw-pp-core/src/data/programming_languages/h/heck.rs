//! Hello-world snippet for `ProgrammingLanguageName::Heck`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Heck>
//! - <https://esolangs.org/wiki/Heck>

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, StackBased};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Heck => "Heck",
    versions: [Heck => "Heck"],
    extensions: [],
}

const HECK: &str = r"6488645864C864C864F86208657864F8652864C86448";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Heck,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative, StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Heck,
            syntax: HECK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HECK,
                    [(
                        TokenKind::Plain,
                        r"6488645864C864C864F86208657864F8652864C86448",
                    )],
                )
            },
        }],
    };
