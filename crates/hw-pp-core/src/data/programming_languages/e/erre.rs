//! Hello-world snippet for `ProgrammingLanguageName::Erre`.
//!
//! Imported from the Hello World Collection (`files/ERRE.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/ERRE.htm>
//! - <http://helloworldcollection.de/#ERRE>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Erre => "ERRE",
    versions: [Erre => "ERRE"],
    extensions: [],
}

const ERRE: &str = concat!(
    r"! ",
    output_for_hello_world!(),
    r#" in ERRE

PROGRAM HELLO
BEGIN
  PRINT(""#,
    output_for_hello_world!(),
    r#"")
END PROGRAM"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Erre,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Erre,
            syntax: ERRE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ERRE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"! ",
                            output_for_hello_world!(),
                            r#" in ERRE

PROGRAM HELLO
BEGIN
  PRINT(""#,
                            output_for_hello_world!(),
                            r#"")
END PROGRAM"#
                        ),
                    )],
                )
            },
        }],
    };
