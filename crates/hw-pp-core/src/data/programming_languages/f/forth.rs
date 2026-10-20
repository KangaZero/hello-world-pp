//! Hello-world snippet for `ProgrammingLanguageName::Forth`.
//!
//! Imported from leachim6/hello-world (`f/Forth.fth` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Forth.fth>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Forth, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Forth => "Forth",
    versions: [Forth => "Forth"],
    extensions: [Fth => "fth"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Forth,
        extensions: &[FileExtension::Fth],
        categories: &[Scripting],
        families: &[Forth],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Forth,
            syntax: concat!(r".( ", output_for_hello_world!(), r")"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r".( ", output_for_hello_world!(), r")"),
            )],
        }],
    };
