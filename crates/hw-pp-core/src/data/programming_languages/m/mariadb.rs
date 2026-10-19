//! Hello-world snippet for `ProgrammingLanguageName::Mariadb`.
//!
//! Imported from leachim6/hello-world (PR #1633 `m/MariaDB.sql`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1633>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Query, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Mariadb => "MariaDB",
    versions: [Mariadb => "MariaDB"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mariadb,
        extensions: &[],
        categories: &[Query],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mariadb,
            syntax: concat!(r#"SELECT ""#, output_for_hello_world!(), r#"";"#),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"SELECT"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Plain, r";"),
            ],
        }],
    };
