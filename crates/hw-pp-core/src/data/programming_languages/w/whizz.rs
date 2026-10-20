//! Hello-world snippet for `ProgrammingLanguageName::Whizz`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Whizz>
//! - <https://esolangs.org/wiki/Whizz>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Whizz => "Whizz",
    versions: [Whizz => "Whizz"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Whizz,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Whizz,
            syntax: r"p72+!29+!7+!!3+!79-!55+!24+!3+!6-!8-!67-!",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"p72+!29+!7+!!3+!79-!55+!24+!3+!6-!8-!67-!",
            )],
        }],
    };
