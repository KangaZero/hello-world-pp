//! Hello-world snippet for `ProgrammingLanguageName::Ppl`.
//!
//! Imported from the Hello World Collection (`files/PPL.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/PPL.txt>
//! - <http://helloworldcollection.de/#PPL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Ppl => "PPL",
    versions: [Ppl => "PPL"],
    extensions: [],
}

const PPL: &str = concat!(
    r";",
    output_for_hello_world!(),
    r#" in PPL (PCBoard Programming Language)
PRINTLN ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ppl,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ppl,
            syntax: PPL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PPL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r";",
                            output_for_hello_world!(),
                            r#" in PPL (PCBoard Programming Language)
PRINTLN ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };
