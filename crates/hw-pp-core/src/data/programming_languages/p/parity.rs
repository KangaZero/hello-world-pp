//! Hello-world snippet for `ProgrammingLanguageName::Parity`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Parity>
//! - <https://esolangs.org/wiki/Parity>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Parity => "Parity",
    versions: [Parity => "Parity"],
    extensions: [],
}

const PARITY: &str = r"72
101
108
108
111
32
87
111
114
108
100
33
10";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Parity,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Parity,
            syntax: PARITY,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PARITY,
                    [(
                        TokenKind::Plain,
                        r"72
101
108
108
111
32
87
111
114
108
100
33
10",
                    )],
                )
            },
        }],
    };
