//! Hello-world snippet for `ProgrammingLanguageName::Phl10`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#PHL_1.0>
//! - <https://esolangs.org/wiki/PHL_1.0>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Phl10 => "PHL 1.0",
    versions: [Phl10 => "PHL 1.0"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Phl10,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Phl10,
            syntax: concat!(
                r"main[
    println(",
                output_for_hello_world!(),
                r");
]"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"main[
    println(",
                    output_for_hello_world!(),
                    r");
]"
                ),
            )],
        }],
    };
