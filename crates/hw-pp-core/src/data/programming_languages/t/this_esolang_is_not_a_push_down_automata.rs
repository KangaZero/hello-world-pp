//! Hello-world snippet for `ProgrammingLanguageName::ThisEsolangIsNotAPushDownAutomata`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#This_esolang_is_not_a_push-down_automata>
//! - <https://esolangs.org/wiki/This_esolang_is_not_a_push-down_automata>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: ThisEsolangIsNotAPushDownAutomata => "This esolang is not a push-down automata",
    versions: [ThisEsolangIsNotAPushDownAutomata => "This esolang is not a push-down automata"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ThisEsolangIsNotAPushDownAutomata,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ThisEsolangIsNotAPushDownAutomata,
            syntax: r"H!e!l!l!o! !W!o!r!l!d!184*+!",
            tokenized_syntax: &[(TokenKind::Plain, r"H!e!l!l!o! !W!o!r!l!d!184*+!")],
        }],
    };
