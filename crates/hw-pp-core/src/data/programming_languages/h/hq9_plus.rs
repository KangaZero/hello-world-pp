//! Hello-world snippet for `ProgrammingLanguageName::Hq9Plus`.
//!
//! Imported from leachim6/hello-world (`h/HQ9+.h` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
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
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hq9Plus,
            syntax: r"H",
            tokenized_syntax: &[(TokenKind::Plain, r"H")],
        }],
    };
