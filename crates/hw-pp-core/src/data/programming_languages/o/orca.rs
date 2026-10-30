//! Hello-world snippet for `ProgrammingLanguageName::Orca`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Orca>
//! - <https://esolangs.org/wiki/Orca>

use crate::data::programming_languages::ProgrammingParadigm::{Reflective, TwoDimensional};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Orca => "Orca",
    versions: [Orca => "Orca"],
    extensions: [],
}

const ORCA: &str = r"Cb............
1bTHello.World";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Orca,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Reflective, TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Orca,
            syntax: ORCA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ORCA,
                    [(
                        TokenKind::Plain,
                        r"Cb............
1bTHello.World",
                    )],
                )
            },
        }],
    };
