//! Hello-world snippet for `ProgrammingLanguageName::Amber`.
//!
//! Imported from leachim6/hello-world (`a/Amber.ab` at `a152253`),
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
    language: Amber => "Amber",
    versions: [Amber => "Amber"],
    extensions: [Ab => "ab"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Amber,
        extensions: &[FileExtension::Ab],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Amber,
            syntax: concat!(r#"puts(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"puts(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
