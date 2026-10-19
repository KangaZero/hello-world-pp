//! Hello-world snippet for `ProgrammingLanguageName::Sil`.
//!
//! Imported from leachim6/hello-world (`s/SIL.SIL` at `a152253`),
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
    language: Sil => "SIL",
    versions: [Sil => "SIL"],
    extensions: [Sil => "SIL"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sil,
        extensions: &[FileExtension::Sil],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sil,
            syntax: concat!(r"print ", output_for_hello_world!()),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"print ", output_for_hello_world!()),
            )],
        }],
    };
