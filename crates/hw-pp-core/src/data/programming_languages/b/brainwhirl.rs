//! Hello-world snippet for `ProgrammingLanguageName::Brainwhirl`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Brainwhirl>
//! - <https://esolangs.org/wiki/Brainwhirl>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Brainwhirl => "Brainwhirl",
    versions: [Brainwhirl => "Brainwhirl"],
    extensions: [],
}

const BRAINWHIRL: &str =
    r"643035560625000020266026004757400664040200424160005200610000073067400066";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Brainwhirl,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Brainwhirl,
            syntax: BRAINWHIRL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BRAINWHIRL,
                    [(
                        TokenKind::Plain,
                        r"643035560625000020266026004757400664040200424160005200610000073067400066",
                    )],
                )
            },
        }],
    };
