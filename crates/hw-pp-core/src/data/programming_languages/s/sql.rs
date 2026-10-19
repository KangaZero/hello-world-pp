//! Hello-world snippets for `ProgrammingLanguageName::Sql`.
//!
//! Sources:
//! - <https://en.wikipedia.org/wiki/SQL:2016>
//! - <https://www.postgresql.org/docs/current/sql-select.html>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Query, Scripting},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Sql => "SQL",
    versions: [Sql2016 => "SQL:2016"],
    extensions: [Sql => "sql"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sql,
        extensions: &[FileExtension::Sql],
        categories: &[Query, Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sql2016,
            syntax: concat!(r"SELECT '", output_for_hello_world!(), r"' AS greeting;"),
            tokenized_syntax: &[
                (TokenKind::Keyword, "SELECT"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("'", output_for_hello_world!(), "'"),
                ),
                (TokenKind::Plain, " "),
                (TokenKind::Keyword, "AS"),
                (TokenKind::Plain, " greeting;"),
            ],
        }],
    };
