//! Hello-world snippet for `ProgrammingLanguageName::Macmac`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Macmac>
//! - <https://esolangs.org/wiki/Macmac>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Macmac => "Macmac",
    versions: [Macmac => "Macmac"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Macmac,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Macmac,
            syntax: r"put(72)
put(101)
put(108)
put(108)
put(111)
put(44)
put(32)
put(119)
put(111)
put(114)
put(108)
put(100)
put(33)",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"put(72)
put(101)
put(108)
put(108)
put(111)
put(44)
put(32)
put(119)
put(111)
put(114)
put(108)
put(100)
put(33)",
            )],
        }],
    };
