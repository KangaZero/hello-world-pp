//! Hello-world snippet for `ProgrammingLanguageName::Squirrel`.
//!
//! Imported from leachim6/hello-world (`s/Squirrel.nut` at `a152253`),
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
    language: Squirrel => "Squirrel",
    versions: [Squirrel => "Squirrel"],
    extensions: [Nut => "nut"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Squirrel,
        extensions: &[FileExtension::Nut],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Squirrel,
            syntax: concat!(r#"print(""#, output_for_hello_world!(), r#"");"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"print(""#, output_for_hello_world!(), r#"");"#),
            )],
        }],
    };
