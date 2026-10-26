//! Hello-world snippet for `ProgrammingLanguageName::Noflo`.
//!
//! Imported from leachim6/hello-world (`n/NoFlo.fbp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/n/NoFlo.fbp>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Framework,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Noflo => "NoFlo",
    versions: [Noflo => "NoFlo"],
    extensions: [Fbp => "fbp"],
}

const NOFLO: &str = concat!(r"'", output_for_hello_world!(), r"' -> IN Display(Output)");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Noflo,
        extensions: &[FileExtension::Fbp],
        categories: &[Framework],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Noflo,
            syntax: NOFLO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NOFLO,
                    [(
                        TokenKind::Plain,
                        concat!(r"'", output_for_hello_world!(), r"' -> IN Display(Output)"),
                    )],
                )
            },
        }],
    };
