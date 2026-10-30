//! Hello-world snippet for `ProgrammingLanguageName::FlamingThunder`.
//!
//! Imported from the Hello World Collection (`files/Flaming&nbsp;Thunder.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Flaming%26nbsp%3BThunder.txt>
//! - <http://helloworldcollection.de/#Flaming%C2%A0Thunder>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: FlamingThunder => "Flaming Thunder",
    versions: [FlamingThunder => "Flaming Thunder"],
    extensions: [],
}

const FLAMING_THUNDER: &str = concat!(
    r#"# Write ""#,
    output_for_hello_world!(),
    r#"" in Flaming Thunder.

Write ""#,
    output_for_hello_world!(),
    r#""."#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::FlamingThunder,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::FlamingThunder,
            syntax: FLAMING_THUNDER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FLAMING_THUNDER,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"# Write ""#,
                            output_for_hello_world!(),
                            r#"" in Flaming Thunder.

Write ""#,
                            output_for_hello_world!(),
                            r#""."#
                        ),
                    )],
                )
            },
        }],
    };
