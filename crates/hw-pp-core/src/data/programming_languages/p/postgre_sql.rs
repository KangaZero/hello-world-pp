//! Hello-world snippet for `ProgrammingLanguageName::PostgreSql`.
//!
//! Imported from leachim6/hello-world (`p/Postgre SQL.sql` at `a152253`),
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
    language: PostgreSql => "Postgre SQL",
    versions: [PostgreSql => "Postgre SQL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PostgreSql,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PostgreSql,
            syntax: concat!(
                r"SELECT '",
                output_for_hello_world!(),
                r"' AS hello_message;"
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"SELECT"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r"'", output_for_hello_world!(), r"'"),
                ),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"AS"),
                (TokenKind::Plain, r" hello_message;"),
            ],
        }],
    };
