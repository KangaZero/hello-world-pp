//! Hello-world snippet for `ProgrammingLanguageName::Gibberish`.
//!
//! Imported from leachim6/hello-world (`g/Gibberish.gibberish` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/Gibberish.gibberish>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Esolang],
        families: &[],
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
