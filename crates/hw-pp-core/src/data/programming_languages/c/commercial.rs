//! Hello-world snippet for `ProgrammingLanguageName::Commercial`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Commercial>
//! - <https://esolangs.org/wiki/Commercial>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Commercial => "Commercial",
    versions: [Commercial => "Commercial"],
    extensions: [],
}

const COMMERCIAL: &str = concat!(
    r#"""#,
    output_for_hello_world!(),
    r#"" - Satisfied Consumer of Hello
Hello has been selling out worldwide!"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Commercial,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Commercial,
            syntax: COMMERCIAL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    COMMERCIAL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"""#,
                            output_for_hello_world!(),
                            r#"" - Satisfied Consumer of Hello
Hello has been selling out worldwide!"#
                        ),
                    )],
                )
            },
        }],
    };
