//! Hello-world snippet for `ProgrammingLanguageName::Clipper`.
//!
//! Imported from leachim6/hello-world (`c/Clipper.prg` at `a152253`),
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
    language: Clipper => "Clipper",
    versions: [Clipper => "Clipper"],
    extensions: [Prg => "prg"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Clipper,
        extensions: &[FileExtension::Prg],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Clipper,
            syntax: concat!(r#"? ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"? ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
