//! Hello-world snippet for `ProgrammingLanguageName::WordBangCpu`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Word!CPU>
//! - <https://esolangs.org/wiki/Word!CPU>

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: WordBangCpu => "Word!CPU",
    versions: [WordBangCpu => "Word!CPU"],
    extensions: [],
}

const WORD_BANG_CPU: &str = r"40 0A 88 02 40 01 68 02 E0 00 48 65 6C 6C 6F 2C 
20 77 6F 72 6C 64 21 0A 00";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::WordBangCpu,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::WordBangCpu,
            syntax: WORD_BANG_CPU,
            tokenized_syntax: &const {
                checked_token_lengths(
                    WORD_BANG_CPU,
                    [(
                        TokenKind::Plain,
                        r"40 0A 88 02 40 01 68 02 E0 00 48 65 6C 6C 6F 2C 
20 77 6F 72 6C 64 21 0A 00",
                    )],
                )
            },
        }],
    };
