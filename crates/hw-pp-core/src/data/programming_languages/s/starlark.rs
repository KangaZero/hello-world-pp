//! Hello-world snippet for `ProgrammingLanguageName::Starlark`.
//!
//! Imported from leachim6/hello-world (`s/Starlark.star` at `a152253`),
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
    language: Starlark => "Starlark",
    versions: [Starlark => "Starlark"],
    extensions: [Star => "star"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Starlark,
        extensions: &[FileExtension::Star],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Starlark,
            syntax: concat!(r#"print(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"print(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
