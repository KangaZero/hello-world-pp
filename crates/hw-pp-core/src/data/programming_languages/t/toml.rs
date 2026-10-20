//! Hello-world snippet for `ProgrammingLanguageName::Toml`.
//!
//! Imported from leachim6/hello-world (`t/TOML.toml` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/TOML.toml>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Configuration,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Toml => "TOML",
    versions: [Toml => "TOML"],
    extensions: [Toml => "toml"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Toml,
        extensions: &[FileExtension::Toml],
        categories: &[Configuration],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Toml,
            syntax: r#"Hello = "World""#,
            tokenized_syntax: &[
                (TokenKind::Plain, r"Hello = "),
                (TokenKind::String, r#""World""#),
            ],
        }],
    };
