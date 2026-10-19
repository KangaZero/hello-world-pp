//! Hello-world snippet for `ProgrammingLanguageName::Zepto`.
//!
//! Imported from leachim6/hello-world (`z/Zepto.zp` at `a152253`),
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
    language: Zepto => "Zepto",
    versions: [Zepto => "Zepto"],
    extensions: [Zp => "zp"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zepto,
        extensions: &[FileExtension::Zp],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zepto,
            syntax: concat!(r#"(write ""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"(write ""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
