//! Hello-world snippet for `ProgrammingLanguageName::MCode`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#M-code>
//! - <https://esolangs.org/wiki/M-Code>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: MCode => "M-code",
    versions: [MCode => "M-code"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::MCode,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MCode,
            syntax: r"4[28]j[4]}[15].i[5]}[5]?[4];_Hello,[32]world!",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"4[28]j[4]}[15].i[5]}[5]?[4];_Hello,[32]world!",
            )],
        }],
    };
