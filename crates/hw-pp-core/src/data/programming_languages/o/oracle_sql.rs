//! Hello-world snippet for `ProgrammingLanguageName::OracleSql`.
//!
//! Imported from leachim6/hello-world (`o/Oracle SQL.sql` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/Oracle%20SQL.sql>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Query, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: OracleSql => "Oracle SQL",
    versions: [OracleSql => "Oracle SQL"],
    extensions: [],
}

const ORACLE_SQL: &str = concat!(r"SELECT '", output_for_hello_world!(), r"' FROM DUAL;");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::OracleSql,
        extensions: &[],
        categories: &[Query],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::OracleSql,
            syntax: ORACLE_SQL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ORACLE_SQL,
                    [
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
                )
            },
        }],
    };
