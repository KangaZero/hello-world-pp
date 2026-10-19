//! Hello-world snippet for `ProgrammingLanguageName::U30c8U30eaU30c8U30eb`.
//!
//! Imported from leachim6/hello-world (`#/ドリトル.dtl` at `a152253`),
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
    language: U30c8U30eaU30c8U30eb => "ドリトル",
    versions: [U30c8U30eaU30c8U30eb => "ドリトル"],
    extensions: [Dtl => "dtl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::U30c8U30eaU30c8U30eb,
        extensions: &[FileExtension::Dtl],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U30c8U30eaU30c8U30eb,
            syntax: concat!(r#"ラベル！（""#, output_for_hello_world!(), r#""）作る。"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"ラベル！（""#, output_for_hello_world!(), r#""）作る。"#),
            )],
        }],
    };
