//! Hello-world snippet for `ProgrammingLanguageName::Rockstar`.
//!
//! Imported from leachim6/hello-world (`r/Rockstar.rock` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Rockstar.rock>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Rockstar => "Rockstar",
    versions: [Rockstar => "Rockstar"],
    extensions: [Rock => "rock"],
}

const ROCKSTAR: &str = concat!(r#"Scream ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rockstar,
        extensions: &[FileExtension::Rock],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rockstar,
            syntax: ROCKSTAR,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ROCKSTAR,
                    [(
                        TokenKind::Plain,
                        concat!(r#"Scream ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
