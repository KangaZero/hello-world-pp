//! Hello-world snippet for `ProgrammingLanguageName::ShockwaveFlash`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Shockwave_flash>
//! - <https://esolangs.org/wiki/Shockwave_flash>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: ShockwaveFlash => "Shockwave flash",
    versions: [ShockwaveFlash => "Shockwave flash"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ShockwaveFlash,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ShockwaveFlash,
            syntax: r"ḩ",
            tokenized_syntax: &[(TokenKind::Plain, r"ḩ")],
        }],
    };
