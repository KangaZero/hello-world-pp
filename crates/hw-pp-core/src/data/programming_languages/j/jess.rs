//! Hello-world snippet for `ProgrammingLanguageName::Jess`.
//!
//! Imported from leachim6/hello-world (`j/Jess.cpl` at `a152253`),
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
    language: Jess => "Jess",
    versions: [Jess => "Jess"],
    extensions: [Cpl => "cpl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Jess,
        extensions: &[FileExtension::Cpl],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Jess,
            syntax: concat!(r#"(printout t ""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"(printout t ""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
