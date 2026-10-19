//! Hello-world snippet for `ProgrammingLanguageName::Chavascript`.
//!
//! Imported from leachim6/hello-world (`c/ChavaScript.chs` at `a152253`),
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
    language: Chavascript => "ChavaScript",
    versions: [Chavascript => "ChavaScript"],
    extensions: [Chs => "chs"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Chavascript,
        extensions: &[FileExtension::Chs],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Chavascript,
            syntax: concat!(r#"בקרה.תעד(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"בקרה.תעד(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
