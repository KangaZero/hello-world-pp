//! Hello-world snippet for `ProgrammingLanguageName::Trigger`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Trigger>
//! - <https://esolangs.org/wiki/Trigger>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Trigger => "Trigger",
    versions: [Trigger => "Trigger"],
    extensions: [],
}

const TRIGGER: &str = r"HHHeeelll lllooo   wwwooorrrlllddd!!!";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Trigger,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Trigger,
            syntax: TRIGGER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TRIGGER,
                    [(TokenKind::Plain, r"HHHeeelll lllooo   wwwooorrrlllddd!!!")],
                )
            },
        }],
    };
