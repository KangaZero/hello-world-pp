//! Hello-world snippet for `ProgrammingLanguageName::Hq9Plus`.
//!
//! Imported from leachim6/hello-world (`h/HQ9+.h` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/HQ9%2B.h>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Hq9Plus => "HQ9+",
    versions: [Hq9Plus => "HQ9+"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hq9Plus,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hq9Plus,
            syntax: r"H",
            tokenized_syntax: &[(TokenKind::Plain, r"H")],
        }],
    };
