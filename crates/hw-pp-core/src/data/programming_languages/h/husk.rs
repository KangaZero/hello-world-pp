//! Hello-world snippet for `ProgrammingLanguageName::Husk`.
//!
//! Imported from leachim6/hello-world (`h/Husk.hs` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/Husk.hs>
//! - <https://esolangs.org/wiki/Husk> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Husk => "Husk",
    versions: [Husk => "Husk"],
    extensions: [],
}

const HUSK: &str = r"¨H◄⁰ω]";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Husk,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Husk,
            syntax: HUSK,
            tokenized_syntax: &const {
                checked_token_lengths(HUSK, [(TokenKind::Plain, r"¨H◄⁰ω]")])
            },
        }],
    };
