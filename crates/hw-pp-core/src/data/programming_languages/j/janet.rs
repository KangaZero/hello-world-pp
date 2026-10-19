//! Hello-world snippet for `ProgrammingLanguageName::Janet`.
//!
//! Imported from leachim6/hello-world (PR #1837 `j/Janet.janet`),
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
    language: Janet => "Janet",
    versions: [Janet => "Janet"],
    extensions: [Janet => "janet"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Janet,
        extensions: &[FileExtension::Janet],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Janet,
            syntax: concat!(r#"(print ""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"(print ""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
