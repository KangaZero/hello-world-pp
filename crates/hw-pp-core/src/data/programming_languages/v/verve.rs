//! Hello-world snippet for `ProgrammingLanguageName::Verve`.
//!
//! Imported from leachim6/hello-world (`v/Verve.vrv` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/Verve.vrv>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Verve => "Verve",
    versions: [Verve => "Verve"],
    extensions: [Vrv => "vrv"],
}

const VERVE: &str = concat!(r#"print(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Verve,
        extensions: &[FileExtension::Vrv],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Verve,
            syntax: VERVE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VERVE,
                    [(
                        TokenKind::Plain,
                        concat!(r#"print(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };
