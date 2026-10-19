//! Hello-world snippet for `ProgrammingLanguageName::Json`.
//!
//! Imported from leachim6/hello-world (`j/JSON.json` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/j/JSON.json>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Json => "JSON",
    versions: [Json => "JSON"],
    extensions: [Json => "json"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Json,
        extensions: &[FileExtension::Json],
        categories: &[Markup],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Json,
            syntax: r#"{ "hello": "world" }"#,
            tokenized_syntax: &[
                (TokenKind::Punctuation, r"{"),
                (TokenKind::Plain, r" "),
                (TokenKind::String, r#""hello""#),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r" "),
                (TokenKind::String, r#""world""#),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"}"),
            ],
        }],
    };
