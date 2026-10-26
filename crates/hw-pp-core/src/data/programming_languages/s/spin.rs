//! Hello-world snippet for `ProgrammingLanguageName::Spin`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Spin>
//! - <https://esolangs.org/wiki/Spin>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Spin => "Spin",
    versions: [Spin => "Spin"],
    extensions: [],
}

const SPIN: &str = r"START TO SPIN!
SPIN THIS ALL! High end lock likes offering. With old red lamps dodged.
STOP SPINNING!";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Spin,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Spin,
            syntax: SPIN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SPIN,
                    [(
                        TokenKind::Plain,
                        r"START TO SPIN!
SPIN THIS ALL! High end lock likes offering. With old red lamps dodged.
STOP SPINNING!",
                    )],
                )
            },
        }],
    };
