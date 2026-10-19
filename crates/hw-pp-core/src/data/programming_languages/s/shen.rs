//! Hello-world snippet for `ProgrammingLanguageName::Shen`.
//!
//! Imported from leachim6/hello-world (`s/Shen.shen` at `a152253`),
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
    language: Shen => "Shen",
    versions: [Shen => "Shen"],
    extensions: [Shen => "shen"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Shen,
        extensions: &[FileExtension::Shen],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Shen,
            syntax: concat!(r#"(pr ""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"(pr ""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
