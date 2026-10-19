//! Hello-world snippet for `ProgrammingLanguageName::Rockstar`.
//!
//! Imported from leachim6/hello-world (`r/Rockstar.rock` at `a152253`),
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
    language: Rockstar => "Rockstar",
    versions: [Rockstar => "Rockstar"],
    extensions: [Rock => "rock"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rockstar,
        extensions: &[FileExtension::Rock],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rockstar,
            syntax: concat!(r#"Scream ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"Scream ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
