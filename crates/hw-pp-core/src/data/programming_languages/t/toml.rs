//! Hello-world snippet for `ProgrammingLanguageName::Toml`.
//!
//! Imported from leachim6/hello-world (`t/TOML.toml` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
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
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Toml,
            syntax: r#"Hello = "World""#,
            tokenized_syntax: &[(TokenKind::Plain, r#"Hello = "World""#)],
        }],
    };
