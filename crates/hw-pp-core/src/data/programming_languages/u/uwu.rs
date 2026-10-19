//! Hello-world snippet for `ProgrammingLanguageName::Uwu`.
//!
//! Imported from leachim6/hello-world (`u/uwu.uwupp` at `a152253`),
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
    language: Uwu => "uwu",
    versions: [Uwu => "uwu"],
    extensions: [Uwupp => "uwupp"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Uwu,
        extensions: &[FileExtension::Uwupp],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Uwu,
            syntax: concat!(r#"nuzzels(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"nuzzels(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
