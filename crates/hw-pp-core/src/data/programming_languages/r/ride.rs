//! Hello-world snippet for `ProgrammingLanguageName::Ride`.
//!
//! Imported from leachim6/hello-world (`r/Ride.ride` at `a152253`),
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
    language: Ride => "Ride",
    versions: [Ride => "Ride"],
    extensions: [Ride => "ride"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ride,
        extensions: &[FileExtension::Ride],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ride,
            syntax: concat!(r#"throw(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"throw(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
