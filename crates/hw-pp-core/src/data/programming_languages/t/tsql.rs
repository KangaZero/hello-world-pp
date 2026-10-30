//! Hello-world snippet for `ProgrammingLanguageName::Tsql`.
//!
//! Imported from leachim6/hello-world (`t/TSQL.sql` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/TSQL.sql>
//! - <https://www.wikidata.org/wiki/Q1411245> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Procedural;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Query, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Tsql => "TSQL",
    versions: [Tsql => "TSQL"],
    extensions: [],
}

const TSQL: &str = concat!(
    r"DECLARE @message varchar(128)
SELECT  @message = '",
    output_for_hello_world!(),
    r"'
PRINT   @message"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tsql,
        extensions: &[],
        categories: &[Query],
        families: &[],
        paradigms: &[Procedural],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tsql,
            syntax: TSQL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TSQL,
                    [
                        (TokenKind::Plain, r"DECLARE @message "),
                        (TokenKind::Type, r"varchar"),
                        (TokenKind::Plain, r"("),
                        (TokenKind::Number, r"128"),
                        (
                            TokenKind::Plain,
                            r")
",
                        ),
                        (TokenKind::Keyword, r"SELECT"),
                        (TokenKind::Plain, r"  @message "),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                        (
                            TokenKind::Plain,
                            r"
PRINT   @message",
                        ),
                    ],
                )
            },
        }],
    };
