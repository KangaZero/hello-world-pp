//! Hello-world snippet for `ProgrammingLanguageName::Pyth`.
//!
//! Imported from leachim6/hello-world (`p/Pyth.pyth` at `a152253`),
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
    language: Pyth => "Pyth",
    versions: [Pyth => "Pyth"],
    extensions: [Pyth => "pyth"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pyth,
        extensions: &[FileExtension::Pyth],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pyth,
            syntax: concat!(r#"""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
