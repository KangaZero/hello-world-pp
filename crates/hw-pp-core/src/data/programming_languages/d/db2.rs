//! Hello-world snippet for `ProgrammingLanguageName::Db2`.
//!
//! Imported from leachim6/hello-world (`d/Db2.sql` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/Db2.sql>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Query, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Db2 => "Db2",
    versions: [Db2 => "Db2"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Db2,
        extensions: &[],
        categories: &[Query],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Db2,
            syntax: concat!(r"VALUES('", output_for_hello_world!(), r"')"),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"VALUES"),
                (TokenKind::Plain, r"("),
                (
                    TokenKind::String,
                    concat!(r"'", output_for_hello_world!(), r"'"),
                ),
                (TokenKind::Plain, r")"),
            ],
        }],
    };
