//! Hello-world snippet for `ProgrammingLanguageName::Crazy`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Crazy>
//! - <https://esolangs.org/wiki/Crazy>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Crazy => "Crazy",
    versions: [Crazy => "Crazy"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Crazy,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Crazy,
            syntax: r" kirmg(s + v + o + o + l + xln + yoz + d + l + i + o + w + vcx)",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r" kirmg(s + v + o + o + l + xln + yoz + d + l + i + o + w + vcx)",
            )],
        }],
    };
