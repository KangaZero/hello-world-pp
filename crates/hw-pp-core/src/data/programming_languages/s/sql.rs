//! Hello-world snippets for `ProgrammingLanguageName::Sql`.
//!
//! Sources:
//! - <https://en.wikipedia.org/wiki/SQL:2016>
//! - <https://www.postgresql.org/docs/current/sql-select.html>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/SQL%26nbsp%3B%28Advantage%29.txt>
//! - <http://helloworldcollection.de/#SQL%C2%A0(Advantage)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/SQL%26nbsp%3B%28DB2%29.txt>
//! - <http://helloworldcollection.de/#SQL%C2%A0(DB2)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/SQL%26nbsp%3B%28Oracle%29.txt>
//! - <http://helloworldcollection.de/#SQL%C2%A0(Oracle)>
//! - <https://www.wikidata.org/wiki/Q47607> (paradigms)
//!
//! The `SQL (Advantage)`, `SQL (DB2)`, `SQL (Oracle)` versions are imported from the Hello World
//! Collection, MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.

use crate::data::programming_languages::ProgrammingParadigm::Declarative;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Query, Scripting},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Sql => "SQL",
    versions: [Sql2016 => "SQL:2016", SqlAdvantage => "SQL (Advantage)", SqlDb2 => "SQL (DB2)", SqlOracle => "SQL (Oracle)"],
    extensions: [Sql => "sql"],
}

const SQL2016: &str = concat!(r"SELECT '", output_for_hello_world!(), r"' AS greeting;");
const SQL_ADVANTAGE: &str = concat!(
    r"-- ",
    output_for_hello_world!(),
    r" in SQL for Advantage Database
 
select '",
    output_for_hello_world!(),
    r"' from system.iota"
);
const SQL_DB2: &str = concat!(
    r"-- ",
    output_for_hello_world!(),
    r" in SQL for DB2
VALUES('",
    output_for_hello_world!(),
    r"')"
);
const SQL_ORACLE: &str = concat!(
    r"# ",
    output_for_hello_world!(),
    r" in SQL for Oracle

SELECT '",
    output_for_hello_world!(),
    r"' FROM dual;"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sql,
        extensions: &[FileExtension::Sql],
        categories: &[Query, Scripting],
        families: &[],
        paradigms: &[Declarative],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Sql2016,
                syntax: SQL2016,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        SQL2016,
                        [
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
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::SqlAdvantage,
                syntax: SQL_ADVANTAGE,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        SQL_ADVANTAGE,
                        [
                            (
                                TokenKind::Comment,
                                concat!(
                                    r"-- ",
                                    output_for_hello_world!(),
                                    r" in SQL for Advantage Database
"
                                ),
                            ),
                            (
                                TokenKind::Plain,
                                r" 
",
                            ),
                            (TokenKind::Keyword, r"select"),
                            (TokenKind::Plain, r" "),
                            (
                                TokenKind::String,
                                concat!(r"'", output_for_hello_world!(), r"'"),
                            ),
                            (TokenKind::Plain, r" "),
                            (TokenKind::Keyword, r"from"),
                            (TokenKind::Plain, r" system.iota"),
                        ],
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::SqlDb2,
                syntax: SQL_DB2,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        SQL_DB2,
                        [
                            (
                                TokenKind::Comment,
                                concat!(
                                    r"-- ",
                                    output_for_hello_world!(),
                                    r" in SQL for DB2
"
                                ),
                            ),
                            (TokenKind::Keyword, r"VALUES"),
                            (TokenKind::Plain, r"("),
                            (
                                TokenKind::String,
                                concat!(r"'", output_for_hello_world!(), r"'"),
                            ),
                            (TokenKind::Plain, r")"),
                        ],
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::SqlOracle,
                syntax: SQL_ORACLE,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        SQL_ORACLE,
                        [
                            (
                                TokenKind::Comment,
                                concat!(
                                    r"# ",
                                    output_for_hello_world!(),
                                    r" in SQL for Oracle
"
                                ),
                            ),
                            (
                                TokenKind::Plain,
                                r"
",
                            ),
                            (TokenKind::Keyword, r"SELECT"),
                            (TokenKind::Plain, r" "),
                            (
                                TokenKind::String,
                                concat!(r"'", output_for_hello_world!(), r"'"),
                            ),
                            (TokenKind::Plain, r" "),
                            (TokenKind::Keyword, r"FROM"),
                            (TokenKind::Plain, r" dual;"),
                        ],
                    )
                },
            },
        ],
    };
