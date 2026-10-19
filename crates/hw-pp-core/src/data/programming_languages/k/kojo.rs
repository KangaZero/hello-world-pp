//! Hello-world snippet for `ProgrammingLanguageName::Kojo`.
//!
//! Imported from leachim6/hello-world (`k/Kojo.kojo` at `a152253`),
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
    language: Kojo => "Kojo",
    versions: [Kojo => "Kojo"],
    extensions: [Kojo => "kojo"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Kojo,
        extensions: &[FileExtension::Kojo],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Kojo,
            syntax: concat!(r#"println(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"println(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
