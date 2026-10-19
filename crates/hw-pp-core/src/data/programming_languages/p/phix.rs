//! Hello-world snippet for `ProgrammingLanguageName::Phix`.
//!
//! Imported from leachim6/hello-world (`p/Phix.exw` at `a152253`),
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
    language: Phix => "Phix",
    versions: [Phix => "Phix"],
    extensions: [Exw => "exw"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Phix,
        extensions: &[FileExtension::Exw],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Phix,
            syntax: concat!(r#"puts(1,""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"puts(1,""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
