//! Hello-world snippet for `ProgrammingLanguageName::Sqlplus`.
//!
//! Imported from the Hello World Collection (`files/sqlplus.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/sqlplus.txt>
//! - <http://helloworldcollection.de/#sqlplus>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Sqlplus => "sqlplus",
    versions: [Sqlplus => "sqlplus"],
    extensions: [],
}

const SQLPLUS: &str = concat!(
    r"-- ",
    output_for_hello_world!(),
    r" in Oracle SQL*Plus

prompt ",
    output_for_hello_world!()
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sqlplus,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sqlplus,
            syntax: SQLPLUS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SQLPLUS,
                    [
                        (
                            TokenKind::Comment,
                            concat!(r"-- ", output_for_hello_world!(), r" in Oracle SQL*Plus"),
                        ),
                        (
                            TokenKind::Plain,
                            concat!(
                                r"

prompt ",
                                output_for_hello_world!()
                            ),
                        ),
                    ],
                )
            },
        }],
    };
