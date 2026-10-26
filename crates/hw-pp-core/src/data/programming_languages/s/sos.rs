//! Hello-world snippet for `ProgrammingLanguageName::Sos`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#SOS>
//! - <https://esolangs.org/wiki/SOS>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Sos => "SOS",
    versions: [Sos => "SOS"],
    extensions: [],
}

const SOS: &str = r"!+!-!!+!-!!!!+!!-!!+!-!+!-!+!!-!+!!-!!!+!!-!+!!-!!!+!!-!+!!!!-!!+!-!!!!!!+!!!-!+!!!-!+!!-!+!!!!-!+!!!-!!+!-!!+!!-!+!!-!!!+!!-!!+!-!!+!-!+!-!";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sos,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sos,
            syntax: SOS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SOS,
                    [(
                        TokenKind::Plain,
                        r"!+!-!!+!-!!!!+!!-!!+!-!+!-!+!!-!+!!-!!!+!!-!+!!-!!!+!!-!+!!!!-!!+!-!!!!!!+!!!-!+!!!-!+!!-!+!!!!-!+!!!-!!+!-!!+!!-!+!!-!!!+!!-!!+!-!!+!-!+!-!",
                    )],
                )
            },
        }],
    };
