//! Hello-world snippet for `ProgrammingLanguageName::Bitsbox`.
//!
//! Imported from leachim6/hello-world (`b/Bitsbox.b` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Bitsbox.b>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Educational, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Bitsbox => "Bitsbox",
    versions: [Bitsbox => "Bitsbox"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bitsbox,
        extensions: &[],
        categories: &[Educational],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bitsbox,
            syntax: concat!(r"text('", output_for_hello_world!(), r"')"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"text('", output_for_hello_world!(), r"')"),
            )],
        }],
    };
