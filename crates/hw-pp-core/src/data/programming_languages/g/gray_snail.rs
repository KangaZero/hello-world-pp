//! Hello-world snippet for `ProgrammingLanguageName::GraySnail`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Gray_Snail>
//! - <https://esolangs.org/wiki/Gray_Snail>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: GraySnail => "Gray Snail",
    versions: [GraySnail => "Gray Snail"],
    extensions: [],
}

const GRAY_SNAIL: &str = concat!(r#"OUTPUT ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::GraySnail,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::GraySnail,
            syntax: GRAY_SNAIL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GRAY_SNAIL,
                    [(
                        TokenKind::Plain,
                        concat!(r#"OUTPUT ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
