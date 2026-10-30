//! Hello-world snippet for `ProgrammingLanguageName::Frak`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#FRAK>
//! - <https://esolangs.org/wiki/FRAK>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Frak => "FRAK",
    versions: [Frak => "FRAK"],
    extensions: [],
}

const FRAK: &str = r"LI 0,C'H'
PUT 0
LI 0,C'e'
PUT 0
LI 0,C'l'
PUT 0
LI 0,C'l'
PUT 0
LI 0,C'o'
PUT 0
LI 0,32
PUT 0
LI 0,C'w'
PUT 0
LI 0,C'o'
PUT 0
LI 0,C'r'
PUT 0
LI 0,C'l'
PUT 0
LI 0,C'd'
PUT 0
LI 0,C'!'
PUT 0";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Frak,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Frak,
            syntax: FRAK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FRAK,
                    [(
                        TokenKind::Plain,
                        r"LI 0,C'H'
PUT 0
LI 0,C'e'
PUT 0
LI 0,C'l'
PUT 0
LI 0,C'l'
PUT 0
LI 0,C'o'
PUT 0
LI 0,32
PUT 0
LI 0,C'w'
PUT 0
LI 0,C'o'
PUT 0
LI 0,C'r'
PUT 0
LI 0,C'l'
PUT 0
LI 0,C'd'
PUT 0
LI 0,C'!'
PUT 0",
                    )],
                )
            },
        }],
    };
