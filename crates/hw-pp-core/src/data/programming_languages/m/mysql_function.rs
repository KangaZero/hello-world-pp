//! Hello-world snippet for `ProgrammingLanguageName::MysqlFunction`.
//!
//! Imported from the Hello World Collection (`files/MySQL&nbsp;FUNCTION.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MySQL%26nbsp%3BFUNCTION.htm>
//! - <http://helloworldcollection.de/#MySQL%C2%A0FUNCTION>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: MysqlFunction => "MySQL FUNCTION",
    versions: [MysqlFunction => "MySQL FUNCTION"],
    extensions: [],
}

const MYSQL_FUNCTION: &str = concat!(
    r"-- ",
    output_for_hello_world!(),
    r" in MySQL FUNCTION

DELIMITER $$
CREATE FUNCTION hello_world() RETURNS TEXT COMMENT '",
    output_for_hello_world!(),
    r"'
BEGIN
  RETURN '",
    output_for_hello_world!(),
    r"';
END;
$$
DELIMITER ;
 
SELECT hello_world();"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::MysqlFunction,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MysqlFunction,
            syntax: MYSQL_FUNCTION,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MYSQL_FUNCTION,
                    [
                        (
                            TokenKind::Comment,
                            concat!(r"-- ", output_for_hello_world!(), r" in MySQL FUNCTION"),
                        ),
                        (
                            TokenKind::Plain,
                            r"

DELIMITER $$
",
                        ),
                        (TokenKind::Keyword, r"CREATE"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"FUNCTION"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"hello_world"),
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"RETURNS"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Type, r"TEXT"),
                        (TokenKind::Plain, r" COMMENT "),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"BEGIN"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Keyword, r"RETURN"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"END"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
$$
DELIMITER ",
                        ),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
 
",
                        ),
                        (TokenKind::Keyword, r"SELECT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"hello_world"),
                        (TokenKind::Punctuation, r"();"),
                    ],
                )
            },
        }],
    };
