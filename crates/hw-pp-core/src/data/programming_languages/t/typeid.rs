//! Hello-world snippet for `ProgrammingLanguageName::Typeid`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Typeid>
//! - <https://esolangs.org/wiki/Typeid>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Typeid => "Typeid",
    versions: [Typeid => "Typeid"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Typeid,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Typeid,
            syntax: r"N5Hello1_IK2aaI1aIhhhhE1bIhhhhhhhhhhhEEK2bbI1cIhhhhE1dIhhhhhhhhEE5worldK2ccI1eIhhhE1fIhhhhhhhhhhhEEEE",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"N5Hello1_IK2aaI1aIhhhhE1bIhhhhhhhhhhhEEK2bbI1cIhhhhE1dIhhhhhhhhEE5worldK2ccI1eIhhhE1fIhhhhhhhhhhhEEEE",
            )],
        }],
    };
