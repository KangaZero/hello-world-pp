//! Hello-world snippet for `ProgrammingLanguageName::Mysql`.
//!
//! Imported from leachim6/hello-world (`m/MySQL.sql` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/MySQL.sql>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Query, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Mysql => "MySQL",
    versions: [Mysql => "MySQL"],
    extensions: [],
}

const MYSQL: &str = concat!(r#"SELECT ""#, output_for_hello_world!(), r#"";"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mysql,
        extensions: &[],
        categories: &[Query],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mysql,
            syntax: MYSQL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MYSQL,
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
