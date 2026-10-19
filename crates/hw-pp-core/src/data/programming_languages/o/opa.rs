//! Hello-world snippet for `ProgrammingLanguageName::Opa`.
//!
//! Imported from leachim6/hello-world (`o/Opa.opa` at `a152253`),
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
    language: Opa => "Opa",
    versions: [Opa => "Opa"],
    extensions: [Opa => "opa"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Opa,
        extensions: &[FileExtension::Opa],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Opa,
            syntax: concat!(r#"jlog(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"jlog(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
