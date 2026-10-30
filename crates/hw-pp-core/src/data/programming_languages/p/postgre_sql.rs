//! Hello-world snippet for `ProgrammingLanguageName::PostgreSql`.
//!
//! Imported from leachim6/hello-world (`p/Postgre SQL.sql` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Postgre%20SQL.sql>
//! - <https://en.wikipedia.org/wiki/SQL> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Declarative;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Query, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: PostgreSql => "Postgre SQL",
    versions: [PostgreSql => "Postgre SQL"],
    extensions: [],
}

const POSTGRE_SQL: &str = concat!(
    r"SELECT '",
    output_for_hello_world!(),
    r"' AS hello_message;"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PostgreSql,
        extensions: &[],
        categories: &[Query],
        families: &[],
        paradigms: &[Declarative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PostgreSql,
            syntax: POSTGRE_SQL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    POSTGRE_SQL,
                    [
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
                )
            },
        }],
    };
