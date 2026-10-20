//! Hello-world snippet for `ProgrammingLanguageName::Cells`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Cells>
//! - <https://esolangs.org/wiki/Cells>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Cells => "Cells",
    versions: [Cells => "Cells"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cells,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cells,
            syntax: r"(alpha)(8,=_)(5,=_)(12,=_)(12,=_)(15,=_)
(ascii)(32,=_)
(alpha)(23,=_)(15,=_)(18,=_)(12,=_)(4,=_)",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"(alpha)(8,=_)(5,=_)(12,=_)(12,=_)(15,=_)
(ascii)(32,=_)
(alpha)(23,=_)(15,=_)(18,=_)(12,=_)(4,=_)",
            )],
        }],
    };
