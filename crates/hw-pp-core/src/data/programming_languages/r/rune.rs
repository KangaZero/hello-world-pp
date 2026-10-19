//! Hello-world snippet for `ProgrammingLanguageName::Rune`.
//!
//! Imported from leachim6/hello-world (`r/Rune.rn` at `a152253`),
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
    language: Rune => "Rune",
    versions: [Rune => "Rune"],
    extensions: [Rn => "rn"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rune,
        extensions: &[FileExtension::Rn],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rune,
            syntax: concat!(r#"print(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"print(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
