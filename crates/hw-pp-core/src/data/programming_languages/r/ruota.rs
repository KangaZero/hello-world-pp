//! Hello-world snippet for `ProgrammingLanguageName::Ruota`.
//!
//! Imported from leachim6/hello-world (`r/Ruota.ruo` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Ruota.ruo>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Ruota => "Ruota",
    versions: [Ruota => "Ruota"],
    extensions: [Ruo => "ruo"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ruota,
        extensions: &[FileExtension::Ruo],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ruota,
            syntax: concat!(r#"printl(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"printl(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
