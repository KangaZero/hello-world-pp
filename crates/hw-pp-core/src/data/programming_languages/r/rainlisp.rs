//! Hello-world snippet for `ProgrammingLanguageName::Rainlisp`.
//!
//! Imported from leachim6/hello-world (PR #1561 `r/RainLisp.rl`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Rainlisp => "RainLisp",
    versions: [Rainlisp => "RainLisp"],
    extensions: [Rl => "rl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rainlisp,
        extensions: &[FileExtension::Rl],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rainlisp,
            syntax: concat!(r#"(display ""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"(display ""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
