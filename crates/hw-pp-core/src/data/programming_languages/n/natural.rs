//! Hello-world snippet for `ProgrammingLanguageName::Natural`.
//!
//! Imported from the Hello World Collection (`files/Natural.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Natural.txt>
//! - <http://helloworldcollection.de/#Natural>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Natural => "Natural",
    versions: [Natural => "Natural"],
    extensions: [],
}

const NATURAL: &str = concat!(
    r" * ",
    output_for_hello_world!(),
    r#" in Natural (by Software AG)
Write ""#,
    output_for_hello_world!(),
    r#""."#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Natural,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Natural,
            syntax: NATURAL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NATURAL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r" * ",
                            output_for_hello_world!(),
                            r#" in Natural (by Software AG)
Write ""#,
                            output_for_hello_world!(),
                            r#""."#
                        ),
                    )],
                )
            },
        }],
    };
