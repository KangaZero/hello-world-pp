//! Hello-world snippet for `ProgrammingLanguageName::Functionable`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Functionable>
//! - <https://esolangs.org/wiki/Functionable>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Functionable => "Functionable",
    versions: [Functionable => "Functionable"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Functionable,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Functionable,
            syntax: r"output(list(72,101,108,108,111,44,32,119,111,115,108,100,33),1)",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"output(list(72,101,108,108,111,44,32,119,111,115,108,100,33),1)",
            )],
        }],
    };
