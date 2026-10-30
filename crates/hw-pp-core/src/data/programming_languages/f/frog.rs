//! Hello-world snippet for `ProgrammingLanguageName::Frog`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Frog>
//! - <https://esolangs.org/wiki/Frog>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Frog => "Frog",
    versions: [Frog => "Frog"],
    extensions: [],
}

const FROG: &str = concat!(r"croak ", output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Frog,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Frog,
            syntax: FROG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FROG,
                    [(
                        TokenKind::Plain,
                        concat!(r"croak ", output_for_hello_world!()),
                    )],
                )
            },
        }],
    };
