//! Hello-world snippet for `ProgrammingLanguageName::Guh`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#guh>
//! - <https://esolangs.org/wiki/Guh>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Guh => "guh",
    versions: [Guh => "guh"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Guh,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Guh,
            syntax: r"guh d l r o W _ o l l e H

guhguhguhguhguh
guhguh
11",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"guh d l r o W _ o l l e H

guhguhguhguhguh
guhguh
11",
            )],
        }],
    };
