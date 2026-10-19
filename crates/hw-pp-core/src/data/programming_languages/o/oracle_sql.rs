//! Hello-world snippet for `ProgrammingLanguageName::OracleSql`.
//!
//! Imported from leachim6/hello-world (`o/Oracle SQL.sql` at `a152253`),
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
    language: OracleSql => "Oracle SQL",
    versions: [OracleSql => "Oracle SQL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::OracleSql,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::OracleSql,
            syntax: concat!(r"SELECT '", output_for_hello_world!(), r"' FROM DUAL;"),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"SELECT"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r"'", output_for_hello_world!(), r"'"),
                ),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"FROM"),
                (TokenKind::Plain, r" DUAL;"),
            ],
        }],
    };
