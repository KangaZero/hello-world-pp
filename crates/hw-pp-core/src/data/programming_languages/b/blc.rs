//! Hello-world snippet for `ProgrammingLanguageName::Blc`.
//!
//! Imported from leachim6/hello-world (`b/BLC.Blc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/BLC.Blc>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Blc => "BLC",
    versions: [Blc => "BLC"],
    extensions: [Blc => "Blc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Blc,
        extensions: &[FileExtension::Blc],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Blc,
            syntax: concat!(r" ", output_for_hello_world!()),
            tokenized_syntax: &[(TokenKind::Plain, concat!(r" ", output_for_hello_world!()))],
        }],
    };
