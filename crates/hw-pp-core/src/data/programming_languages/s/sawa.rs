//! Hello-world snippet for `ProgrammingLanguageName::Sawa`.
//!
//! Imported from leachim6/hello-world (`s/Sawa.ꦱꦮ` at `a152253`),
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
    language: Sawa => "Sawa",
    versions: [Sawa => "Sawa"],
    extensions: [Ua9b1Ua9ae => "ꦱꦮ"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sawa,
        extensions: &[FileExtension::Ua9b1Ua9ae],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sawa,
            syntax: concat!(r#"ꦥꦿꦶꦤ꧀(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"ꦥꦿꦶꦤ꧀(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
