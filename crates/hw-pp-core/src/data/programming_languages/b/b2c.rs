//! Hello-world snippet for `ProgrammingLanguageName::B2c`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#B2C>
//! - <https://esolangs.org/wiki/B2C>

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: B2c => "B2C",
    versions: [B2c => "B2C"],
    extensions: [],
}

const B2C: &str = r"+++++++++[|++++++++|-]|.[-]|++++++++++[|++++++++++|-]|.
+++++++..+++.[-]|++++++++[|++++|-]|.[-]|
+++++++[|+++++++++++|-]|.[-]|++++++++++[|++++++++++|-]|
+.+++.------.--------.+++++++++++[|+++|-]|.";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::B2c,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::B2c,
            syntax: B2C,
            tokenized_syntax: &const {
                checked_token_lengths(
                    B2C,
                    [(
                        TokenKind::Plain,
                        r"+++++++++[|++++++++|-]|.[-]|++++++++++[|++++++++++|-]|.
+++++++..+++.[-]|++++++++[|++++|-]|.[-]|
+++++++[|+++++++++++|-]|.[-]|++++++++++[|++++++++++|-]|
+.+++.------.--------.+++++++++++[|+++|-]|.",
                    )],
                )
            },
        }],
    };
