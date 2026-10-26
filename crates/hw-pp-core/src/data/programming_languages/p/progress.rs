//! Hello-world snippet for `ProgrammingLanguageName::Progress`.
//!
//! Imported from the Hello World Collection (`files/Progress.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Progress.txt>
//! - <http://helloworldcollection.de/#Progress>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Progress => "Progress",
    versions: [Progress => "Progress"],
    extensions: [],
}

const PROGRESS: &str = concat!(
    r"/* ",
    output_for_hello_world!(),
    r#" in Progress */

message ""#,
    output_for_hello_world!(),
    r#"" view-as alert-box."#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Progress,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Progress,
            syntax: PROGRESS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PROGRESS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"/* ",
                            output_for_hello_world!(),
                            r#" in Progress */

message ""#,
                            output_for_hello_world!(),
                            r#"" view-as alert-box."#
                        ),
                    )],
                )
            },
        }],
    };
