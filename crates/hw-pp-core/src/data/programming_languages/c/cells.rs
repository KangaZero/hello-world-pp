//! Hello-world snippet for `ProgrammingLanguageName::Cells`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Cells>
//! - <https://esolangs.org/wiki/Cells>

use crate::data::programming_languages::ProgrammingParadigm::StringRewriting;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Cells => "Cells",
    versions: [Cells => "Cells"],
    extensions: [],
}

const CELLS: &str = r"(alpha)(8,=_)(5,=_)(12,=_)(12,=_)(15,=_)
(ascii)(32,=_)
(alpha)(23,=_)(15,=_)(18,=_)(12,=_)(4,=_)";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cells,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StringRewriting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cells,
            syntax: CELLS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CELLS,
                    [(
                        TokenKind::Plain,
                        r"(alpha)(8,=_)(5,=_)(12,=_)(12,=_)(15,=_)
(ascii)(32,=_)
(alpha)(23,=_)(15,=_)(18,=_)(12,=_)(4,=_)",
                    )],
                )
            },
        }],
    };
