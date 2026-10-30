//! Hello-world snippet for `ProgrammingLanguageName::Cubix`.
//!
//! Imported from leachim6/hello-world (`c/Cubix` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Cubix>
//! - <https://esolangs.org/wiki/Cubix> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{StackBased, TwoDimensional};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Cubix => "Cubix",
    versions: [Cubix => "Cubix"],
    extensions: [],
}

const CUBIX: &str = r#"./v.o;@?/."dlroW"S..u/"Hello""#;

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cubix,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased, TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cubix,
            syntax: CUBIX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CUBIX,
                    [(TokenKind::Plain, r#"./v.o;@?/."dlroW"S..u/"Hello""#)],
                )
            },
        }],
    };
