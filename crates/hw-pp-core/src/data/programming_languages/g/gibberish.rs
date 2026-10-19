//! Hello-world snippet for `ProgrammingLanguageName::Gibberish`.
//!
//! Imported from leachim6/hello-world (`g/Gibberish.gibberish` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Gibberish => "Gibberish",
    versions: [Gibberish => "Gibberish"],
    extensions: [Gibberish => "gibberish"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gibberish,
        extensions: &[FileExtension::Gibberish],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gibberish,
            syntax: concat!(
                r"[",
                output_for_hello_world!(),
                r"
]eo"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"[",
                    output_for_hello_world!(),
                    r"
]eo"
                ),
            )],
        }],
    };
