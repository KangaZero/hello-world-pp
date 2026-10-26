//! Hello-world snippet for `ProgrammingLanguageName::D4`.
//!
//! Imported from the Hello World Collection (`files/D4.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/D4.htm>
//! - <http://helloworldcollection.de/#D4>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: D4 => "D4",
    versions: [D4 => "D4"],
    extensions: [],
}

const D4: &str = concat!(
    r"// ",
    output_for_hello_world!(),
    r#" as a relation-variable in D4

select row { ""#,
    output_for_hello_world!(),
    r#"" AMessage }"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::D4,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::D4,
            syntax: D4,
            tokenized_syntax: &const {
                checked_token_lengths(
                    D4,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r#" as a relation-variable in D4

select row { ""#,
                            output_for_hello_world!(),
                            r#"" AMessage }"#
                        ),
                    )],
                )
            },
        }],
    };
