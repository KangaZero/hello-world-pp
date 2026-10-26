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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Mariadb => "MariaDB",
    versions: [Mariadb => "MariaDB"],
    extensions: [],
}

const MARIADB: &str = concat!(r#"SELECT ""#, output_for_hello_world!(), r#"";"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mariadb,
        extensions: &[],
        categories: &[Query],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mariadb,
            syntax: MARIADB,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MARIADB,
                    [
                        (TokenKind::Keyword, r"SELECT"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Plain, r";"),
                    ],
                )
            },
        }],
    };
