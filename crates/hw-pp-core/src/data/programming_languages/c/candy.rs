//! Hello-world snippet for `ProgrammingLanguageName::Candy`.
//!
//! Imported from leachim6/hello-world (`c/Candy.candy` at `a152253`),
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
    language: Candy => "Candy",
    versions: [Candy => "Candy"],
    extensions: [Candy => "candy"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Candy,
        extensions: &[FileExtension::Candy],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Candy,
            syntax: concat!(r#"""#, output_for_hello_world!(), r#""(;)"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"""#, output_for_hello_world!(), r#""(;)"#),
            )],
        }],
    };
