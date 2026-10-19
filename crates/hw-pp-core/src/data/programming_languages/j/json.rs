//! Hello-world snippet for `ProgrammingLanguageName::Json`.
//!
//! Imported from leachim6/hello-world (`j/JSON.json` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
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
        categories: &[],
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
