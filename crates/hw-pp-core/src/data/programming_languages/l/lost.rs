//! Hello-world snippet for `ProgrammingLanguageName::Lost`.
//!
//! Imported from leachim6/hello-world (`l/Lost.？？？` at `a152253`),
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
    language: Lost => "Lost",
    versions: [Lost => "Lost"],
    extensions: [Uff1fUff1fUff1f => "？？？"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lost,
        extensions: &[FileExtension::Uff1fUff1fUff1f],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lost,
            syntax: concat!(r#"%""#, output_for_hello_world!(), r#""@"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"%""#, output_for_hello_world!(), r#""@"#),
            )],
        }],
    };
