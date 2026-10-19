//! Hello-world snippet for `ProgrammingLanguageName::Emoji`.
//!
//! Imported from leachim6/hello-world (`e/Emoji` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Emoji => "Emoji",
    versions: [Emoji => "Emoji"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Emoji,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Emoji,
            syntax: concat!(r"💬", output_for_hello_world!(), r"💬➡"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"💬", output_for_hello_world!(), r"💬➡"),
            )],
        }],
    };
