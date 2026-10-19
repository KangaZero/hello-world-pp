//! Hello-world snippet for `ProgrammingLanguageName::Zlang`.
//!
//! Imported from leachim6/hello-world (PR #1552 `z/zlang.zlang`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1552>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Zlang => "zlang",
    versions: [Zlang => "zlang"],
    extensions: [Zlang => "zlang"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zlang,
        extensions: &[FileExtension::Zlang],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zlang,
            syntax: concat!(r#"log(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"log(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
