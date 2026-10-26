//! Hello-world snippet for `ProgrammingLanguageName::Mention`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Mention>
//! - <https://esolangs.org/wiki/Mention>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Mention => "Mention",
    versions: [Mention => "Mention"],
    extensions: [],
}

const MENTION: &str = r"v 0 01001000 // H
o 1
v 1 01100101 // e
o 1
v 10 01101100 // l
o 1
o 1
v 11 01101111 // o
o 1
v 100 00100000 // space
o 1
v 101 01010111 // W
o 1
v 11 // o
o 1
v 110 01110000 // r
o 1
v 10 // l
o 1
v 111 01100100 // d
o 1";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mention,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mention,
            syntax: MENTION,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MENTION,
                    [(
                        TokenKind::Plain,
                        r"v 0 01001000 // H
o 1
v 1 01100101 // e
o 1
v 10 01101100 // l
o 1
o 1
v 11 01101111 // o
o 1
v 100 00100000 // space
o 1
v 101 01010111 // W
o 1
v 11 // o
o 1
v 110 01110000 // r
o 1
v 10 // l
o 1
v 111 01100100 // d
o 1",
                    )],
                )
            },
        }],
    };
