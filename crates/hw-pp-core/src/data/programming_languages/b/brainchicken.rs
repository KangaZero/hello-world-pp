//! Hello-world snippet for `ProgrammingLanguageName::Brainchicken`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Brainchicken>
//! - <https://esolangs.org/wiki/Brainchicken>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Brainchicken => "Brainchicken",
    versions: [Brainchicken => "Brainchicken"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Brainchicken,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Brainchicken,
            syntax: r"chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken
chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken
chicken
chicken chicken chicken
chicken chicken chicken
chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken
chicken chicken chicken
chicken chicken
chicken chicken
chicken chicken
chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken
chicken
chicken chicken chicken
chicken
chicken chicken chicken
chicken
chicken chicken chicken chicken
chicken
chicken
chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken
chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken
chicken
chicken
chicken chicken chicken chicken chicken
chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken chicken chicken
chicken
chicken
chicken chicken chicken chicken chicken
chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken chicken
chicken chicken
chicken chicken chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken chicken
chicken
chicken
chicken chicken chicken
chicken chicken chicken chicken chicken
chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken chicken chicken",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken
chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken
chicken
chicken chicken chicken
chicken chicken chicken
chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken
chicken chicken chicken
chicken chicken
chicken chicken
chicken chicken
chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken
chicken
chicken chicken chicken
chicken
chicken chicken chicken
chicken
chicken chicken chicken chicken
chicken
chicken
chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken
chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken
chicken
chicken
chicken chicken chicken chicken chicken
chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken chicken chicken
chicken
chicken
chicken chicken chicken chicken chicken
chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken chicken
chicken chicken
chicken chicken chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken chicken
chicken
chicken
chicken chicken chicken
chicken chicken chicken chicken chicken
chicken
chicken chicken chicken
chicken chicken chicken
chicken chicken chicken chicken chicken",
            )],
        }],
    };
