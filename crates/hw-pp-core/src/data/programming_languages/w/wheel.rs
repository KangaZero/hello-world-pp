//! Hello-world snippet for `ProgrammingLanguageName::Wheel`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Wheel>
//! - <https://esolangs.org/wiki/Wheel>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Wheel => "Wheel",
    versions: [Wheel => "Wheel"],
    extensions: [],
}

const WHEEL: &str = r"C72C69C76C76C79C32C87C79C82C76C68";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Wheel,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Wheel,
            syntax: WHEEL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    WHEEL,
                    [(TokenKind::Plain, r"C72C69C76C76C79C32C87C79C82C76C68")],
                )
            },
        }],
    };
