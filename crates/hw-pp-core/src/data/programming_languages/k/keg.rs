//! Hello-world snippet for `ProgrammingLanguageName::Keg`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Keg>
//! - <https://esolangs.org/wiki/Keg>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Keg => "Keg",
    versions: [Keg => "Keg"],
    extensions: [],
}

const KEG: &str = r"Hello\, World\!";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Keg,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Keg,
            syntax: KEG,
            tokenized_syntax: &const {
                checked_token_lengths(KEG, [(TokenKind::Plain, r"Hello\, World\!")])
            },
        }],
    };
