//! Hello-world snippet for `ProgrammingLanguageName::Spwn`.
//!
//! Imported from leachim6/hello-world (`s/SPWN.spwn` at `a152253`),
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
    language: Spwn => "SPWN",
    versions: [Spwn => "SPWN"],
    extensions: [Spwn => "spwn"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Spwn,
        extensions: &[FileExtension::Spwn],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Spwn,
            syntax: concat!(r#"$.print(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"$.print(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
