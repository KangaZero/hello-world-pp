//! Hello-world snippet for `ProgrammingLanguageName::M4`.
//!
//! Imported from leachim6/hello-world (`m/M4.m4` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/M4.m4>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: M4 => "M4",
    versions: [M4 => "M4"],
    extensions: [M4 => "m4"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::M4,
        extensions: &[FileExtension::M4],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::M4,
            syntax: r"Hello",
            tokenized_syntax: &[(TokenKind::Plain, r"Hello")],
        }],
    };
