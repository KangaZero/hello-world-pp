//! Hello-world snippet for `ProgrammingLanguageName::Attache`.
//!
//! Imported from leachim6/hello-world (`a/Attache.@` at `a152253`),
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
    language: Attache => "Attache",
    versions: [Attache => "Attache"],
    extensions: [At2 => "@"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Attache,
        extensions: &[FileExtension::At2],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Attache,
            syntax: concat!(r#"Print[""#, output_for_hello_world!(), r#""]"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"Print[""#, output_for_hello_world!(), r#""]"#),
            )],
        }],
    };
