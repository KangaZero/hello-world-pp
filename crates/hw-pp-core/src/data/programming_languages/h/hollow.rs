//! Hello-world snippet for `ProgrammingLanguageName::Hollow`.
//!
//! Imported from leachim6/hello-world (`h/Hollow.hollow` at `a152253`),
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
    language: Hollow => "Hollow",
    versions: [Hollow => "Hollow"],
    extensions: [Hollow => "hollow"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hollow,
        extensions: &[FileExtension::Hollow],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hollow,
            syntax: concat!(r"{", output_for_hello_world!(), r".9}:?"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"{", output_for_hello_world!(), r".9}:?"),
            )],
        }],
    };
