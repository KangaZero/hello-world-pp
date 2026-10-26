//! Hello-world snippet for `ProgrammingLanguageName::U03b9U03c7U03b8U03c5U03c2`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#%CE%99%CE%A7%CE%98%CE%A5%CE%A3>
//! - <https://esolangs.org/wiki/%CE%99%CE%A7%CE%98%CE%A5%CE%A3>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: U03b9U03c7U03b8U03c5U03c2 => "ΙΧΘΥΣ",
    versions: [U03b9U03c7U03b8U03c5U03c2 => "ΙΧΘΥΣ"],
    extensions: [],
}

const U03B9_U03C7_U03B8_U03C5_U03C2: &str = r" ιιιθχθιιιιιιιιΥΧιιιθιθιΥιιιιιιιΥΥιιιΥΧιιιθχχθχχχχχΥΧιιθιιθχχχχΥΧιιιθθιιιιιιΥΧιιιθιιθχχχχχχχχχχΥιιιΥχχχχχχΥΧιιιθιθΥΧιιθιιθχχχΥ";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::U03b9U03c7U03b8U03c5U03c2,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U03b9U03c7U03b8U03c5U03c2,
            syntax: U03B9_U03C7_U03B8_U03C5_U03C2,
            tokenized_syntax: &const {
                checked_token_lengths(
                    U03B9_U03C7_U03B8_U03C5_U03C2,
                    [(
                        TokenKind::Plain,
                        r" ιιιθχθιιιιιιιιΥΧιιιθιθιΥιιιιιιιΥΥιιιΥΧιιιθχχθχχχχχΥΧιιθιιθχχχχΥΧιιιθθιιιιιιΥΧιιιθιιθχχχχχχχχχχΥιιιΥχχχχχχΥΧιιιθιθΥΧιιθιιθχχχΥ",
                    )],
                )
            },
        }],
    };
