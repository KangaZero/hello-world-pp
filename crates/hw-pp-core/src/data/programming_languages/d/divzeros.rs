//! Hello-world snippet for `ProgrammingLanguageName::Divzeros`.
//!
//! Imported from leachim6/hello-world (`d/Divzeros.dz` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Divzeros => "Divzeros",
    versions: [Divzeros => "Divzeros"],
    extensions: [Dz => "dz"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Divzeros,
        extensions: &[FileExtension::Dz],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Divzeros,
            syntax: r"(?'H+?'e+?'l+?'l+?'o+?',+?' +?'W+?'o+?'r+?'l+?'d+?'!)/0",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"(?'H+?'e+?'l+?'l+?'o+?',+?' +?'W+?'o+?'r+?'l+?'d+?'!)/0",
            )],
        }],
    };
