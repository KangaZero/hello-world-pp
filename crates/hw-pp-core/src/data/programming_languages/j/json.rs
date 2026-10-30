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
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Json => "JSON",
    versions: [Json => "JSON"],
    extensions: [Json => "json"],
}

const JSON: &str = r#"{ "hello": "world" }"#;

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Json,
        extensions: &[FileExtension::Json],
        categories: &[Markup],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Json,
            syntax: JSON,
            tokenized_syntax: &const {
                checked_token_lengths(
                    JSON,
                    [
                        (TokenKind::Punctuation, r"{"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r#""hello""#),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r#""world""#),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"}"),
                    ],
                )
            },
        }],
    };
