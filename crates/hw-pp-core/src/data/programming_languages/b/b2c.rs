//! Hello-world snippet for `ProgrammingLanguageName::B2c`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#B2C>
//! - <https://esolangs.org/wiki/B2C>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: B2c => "B2C",
    versions: [B2c => "B2C"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::B2c,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::B2c,
            syntax: r"+++++++++[|++++++++|-]|.[-]|++++++++++[|++++++++++|-]|.
+++++++..+++.[-]|++++++++[|++++|-]|.[-]|
+++++++[|+++++++++++|-]|.[-]|++++++++++[|++++++++++|-]|
+.+++.------.--------.+++++++++++[|+++|-]|.",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"+++++++++[|++++++++|-]|.[-]|++++++++++[|++++++++++|-]|.
+++++++..+++.[-]|++++++++[|++++|-]|.[-]|
+++++++[|+++++++++++|-]|.[-]|++++++++++[|++++++++++|-]|
+.+++.------.--------.+++++++++++[|+++|-]|.",
            )],
        }],
    };
