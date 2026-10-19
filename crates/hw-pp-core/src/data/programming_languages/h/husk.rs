//! Hello-world snippet for `ProgrammingLanguageName::Husk`.
//!
//! Imported from leachim6/hello-world (`h/Husk.hs` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Husk => "Husk",
    versions: [Husk => "Husk"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Husk,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Husk,
            syntax: r"¨H◄⁰ω]",
            tokenized_syntax: &[(TokenKind::Plain, r"¨H◄⁰ω]")],
        }],
    };
