//! Hello-world snippet for `ProgrammingLanguageName::Vtl`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#VTL>
//! - <https://esolangs.org/wiki/VTL>

use crate::data::programming_languages::ProgrammingParadigm::{CellBased, Reflective};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Vtl => "VTL",
    versions: [Vtl => "VTL"],
    extensions: [],
}

const VTL: &str = r"1D C4 01 C4 01 C4 01 C4 01 C4 01 C4 01 C4 01 C4
01 C4 01 C4 01 C4 01 C4 01 C4 01 C4 E0 48 65 6C
6C 6F 2C 20 77 6F 72 6C 64 21";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vtl,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Reflective, CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Vtl,
            syntax: VTL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VTL,
                    [(
                        TokenKind::Plain,
                        r"1D C4 01 C4 01 C4 01 C4 01 C4 01 C4 01 C4 01 C4
01 C4 01 C4 01 C4 01 C4 01 C4 01 C4 E0 48 65 6C
6C 6F 2C 20 77 6F 72 6C 64 21",
                    )],
                )
            },
        }],
    };
