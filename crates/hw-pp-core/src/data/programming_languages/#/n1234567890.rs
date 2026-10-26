//! Hello-world snippet for `ProgrammingLanguageName::N1234567890`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#1234567890>
//! - <https://esolangs.org/wiki/1234567890>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: N1234567890 => "1234567890",
    versions: [N1234567890 => "1234567890"],
    extensions: [],
}

const N1234567890: &str = r"---START 01---
hello=0100100001100101011011000110110001101111001000000
1110111011011110111001001101100011001000010000100001010.
---END---";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N1234567890,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N1234567890,
            syntax: N1234567890,
            tokenized_syntax: &const {
                checked_token_lengths(
                    N1234567890,
                    [(
                        TokenKind::Plain,
                        r"---START 01---
hello=0100100001100101011011000110110001101111001000000
1110111011011110111001001101100011001000010000100001010.
---END---",
                    )],
                )
            },
        }],
    };
