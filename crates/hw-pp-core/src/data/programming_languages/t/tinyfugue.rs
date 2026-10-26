//! Hello-world snippet for `ProgrammingLanguageName::Tinyfugue`.
//!
//! Imported from the Hello World Collection (`files/TinyFugue.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/TinyFugue.htm>
//! - <http://helloworldcollection.de/#TinyFugue>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Tinyfugue => "TinyFugue",
    versions: [Tinyfugue => "TinyFugue"],
    extensions: [],
}

const TINYFUGUE: &str = concat!(
    r";",
    output_for_hello_world!(),
    r" in TinyFugue

/_echo ",
    output_for_hello_world!()
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tinyfugue,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tinyfugue,
            syntax: TINYFUGUE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TINYFUGUE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r";",
                            output_for_hello_world!(),
                            r" in TinyFugue

/_echo ",
                            output_for_hello_world!()
                        ),
                    )],
                )
            },
        }],
    };
