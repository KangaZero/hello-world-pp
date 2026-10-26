//! Hello-world snippet for `ProgrammingLanguageName::Yacas`.
//!
//! Imported from the Hello World Collection (`files/Yacas.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Yacas.htm>
//! - <http://helloworldcollection.de/#Yacas>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Yacas => "Yacas",
    versions: [Yacas => "Yacas"],
    extensions: [],
}

const YACAS: &str = concat!(
    output_for_hello_world!(),
    r#" in Yacas

WriteString(""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Yacas,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Yacas,
            syntax: YACAS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    YACAS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            output_for_hello_world!(),
                            r#" in Yacas

WriteString(""#,
                            output_for_hello_world!(),
                            r#"")"#
                        ),
                    )],
                )
            },
        }],
    };
