//! Hello-world snippet for `ProgrammingLanguageName::Defunc`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Defunc>
//! - <https://esolangs.org/wiki/Defunc>

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Defunc => "Defunc",
    versions: [Defunc => "Defunc"],
    extensions: [],
}

const DEFUNC: &str = r"4a++++a
Ba44444444a
LBBB4440
.+4444444.BB440
..L
.+++L
.+++44444BB.B0
.+++.+++L
.L
.BBB40
.+B0";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Defunc,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Defunc,
            syntax: DEFUNC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DEFUNC,
                    [(
                        TokenKind::Plain,
                        r"4a++++a
Ba44444444a
LBBB4440
.+4444444.BB440
..L
.+++L
.+++44444BB.B0
.+++.+++L
.L
.BBB40
.+B0",
                    )],
                )
            },
        }],
    };
