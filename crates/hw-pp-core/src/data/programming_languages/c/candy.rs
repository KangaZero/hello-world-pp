//! Hello-world snippet for `ProgrammingLanguageName::Candy`.
//!
//! Imported from leachim6/hello-world (`c/Candy.candy` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Candy.candy>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Candy => "Candy",
    versions: [Candy => "Candy"],
    extensions: [Candy => "candy"],
}

const CANDY: &str = concat!(r#"""#, output_for_hello_world!(), r#""(;)"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Candy,
        extensions: &[FileExtension::Candy],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Candy,
            syntax: CANDY,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CANDY,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#""(;)"#),
                    )],
                )
            },
        }],
    };
