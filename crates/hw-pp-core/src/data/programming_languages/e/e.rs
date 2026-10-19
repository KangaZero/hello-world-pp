//! Hello-world snippet for `ProgrammingLanguageName::E`.
//!
//! Imported from leachim6/hello-world (`e/E.e` at `a152253`),
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
    language: E => "E",
    versions: [E => "E"],
    extensions: [E => "e"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::E,
        extensions: &[FileExtension::E],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::E,
            syntax: concat!(r#"println(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"println(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
