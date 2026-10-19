//! Hello-world snippet for `ProgrammingLanguageName::Logo`.
//!
//! Imported from leachim6/hello-world (`l/Logo.lg` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/Logo.lg>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Educational,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Logo => "Logo",
    versions: [Logo => "Logo"],
    extensions: [Lg => "lg"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Logo,
        extensions: &[FileExtension::Lg],
        categories: &[Educational],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Logo,
            syntax: concat!(r"print [", output_for_hello_world!(), r"]"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"print [", output_for_hello_world!(), r"]"),
            )],
        }],
    };
