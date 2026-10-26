//! Hello-world snippet for `ProgrammingLanguageName::Keyf`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#KeyF>
//! - <https://esolangs.org/wiki/KeyF>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Keyf => "KeyF",
    versions: [Keyf => "KeyF"],
    extensions: [],
}

const KEYF: &str = r">v>v,<^<^<^<.v>v>v>v>v>v>v..<.^^<^^.<<<,v>v>v>v>v>v>v>.^<^<^<^<^<.v>v>v>v>v>v.<^<^<^<^<^<^.<^<^<<,vvv>v>v>v>v>v>v>";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Keyf,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Keyf,
            syntax: KEYF,
            tokenized_syntax: &const {
                checked_token_lengths(
                    KEYF,
                    [(
                        TokenKind::Plain,
                        r">v>v,<^<^<^<.v>v>v>v>v>v>v..<.^^<^^.<<<,v>v>v>v>v>v>v>.^<^<^<^<^<.v>v>v>v>v>v.<^<^<^<^<^<^.<^<^<<,vvv>v>v>v>v>v>v>",
                    )],
                )
            },
        }],
    };
