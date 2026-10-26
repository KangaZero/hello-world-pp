//! Hello-world snippet for `ProgrammingLanguageName::Bicep`.
//!
//! Imported from leachim6/hello-world (`b/Bicep.bicep` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Bicep.bicep>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Configuration,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Bicep => "Bicep",
    versions: [Bicep => "Bicep"],
    extensions: [Bicep => "bicep"],
}

const BICEP: &str = concat!(r"output msg string = '", output_for_hello_world!(), r"'");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bicep,
        extensions: &[FileExtension::Bicep],
        categories: &[Configuration],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bicep,
            syntax: BICEP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BICEP,
                    [
                        (TokenKind::Plain, r"output msg "),
                        (TokenKind::Type, r"string"),
                        (TokenKind::Plain, r" = "),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                    ],
                )
            },
        }],
    };
