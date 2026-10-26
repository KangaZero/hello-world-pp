//! Hello-world snippet for `ProgrammingLanguageName::Judoscript`.
//!
//! Imported from leachim6/hello-world (`j/JudoScript` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/j/JudoScript>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Judoscript => "JudoScript",
    versions: [Judoscript => "JudoScript"],
    extensions: [],
}

const JUDOSCRIPT: &str = concat!(r#". ""#, output_for_hello_world!(), r#"";"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Judoscript,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Judoscript,
            syntax: JUDOSCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    JUDOSCRIPT,
                    [(
                        TokenKind::Plain,
                        concat!(r#". ""#, output_for_hello_world!(), r#"";"#),
                    )],
                )
            },
        }],
    };
