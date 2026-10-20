//! Hello-world snippet for `ProgrammingLanguageName::Cson`.
//!
//! Imported from leachim6/hello-world (`c/CSON.cson` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/CSON.cson>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Cson => "CSON",
    versions: [Cson => "CSON"],
    extensions: [Cson => "cson"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cson,
        extensions: &[FileExtension::Cson],
        categories: &[Markup],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cson,
            syntax: r"{'Hello': 'World'}",
            tokenized_syntax: &[(TokenKind::Plain, r"{'Hello': 'World'}")],
        }],
    };
