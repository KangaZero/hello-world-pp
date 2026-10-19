//! Hello-world snippet for `ProgrammingLanguageName::Fennel`.
//!
//! Imported from leachim6/hello-world (`f/Fennel.fnl` at `a152253`),
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
    language: Fennel => "Fennel",
    versions: [Fennel => "Fennel"],
    extensions: [Fnl => "fnl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fennel,
        extensions: &[FileExtension::Fnl],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fennel,
            syntax: concat!(r#"(print ""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"(print ""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
