//! Hello-world snippet for `ProgrammingLanguageName::WordBangCpu`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Word!CPU>
//! - <https://esolangs.org/wiki/Word!CPU>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: WordBangCpu => "Word!CPU",
    versions: [WordBangCpu => "Word!CPU"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::WordBangCpu,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::WordBangCpu,
            syntax: r"40 0A 88 02 40 01 68 02 E0 00 48 65 6C 6C 6F 2C 
20 77 6F 72 6C 64 21 0A 00",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"40 0A 88 02 40 01 68 02 E0 00 48 65 6C 6C 6F 2C 
20 77 6F 72 6C 64 21 0A 00",
            )],
        }],
    };
