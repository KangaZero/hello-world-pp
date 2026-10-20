//! Hello-world snippet for `ProgrammingLanguageName::Backflip`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#BackFlip>
//! - <https://esolangs.org/wiki/BackFlip>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Backflip => "BackFlip",
    versions: [Backflip => "BackFlip"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Backflip,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Backflip,
            syntax: r"\72N101N108N108N111N44N32N119N111N114N108N100N33N10N
\///////////////////////////////////////////////////",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"\72N101N108N108N111N44N32N119N111N114N108N100N33N10N
\///////////////////////////////////////////////////",
            )],
        }],
    };
