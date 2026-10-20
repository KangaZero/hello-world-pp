//! Hello-world snippet for `ProgrammingLanguageName::SuperStackBang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Super_Stack!>
//! - <https://esolangs.org/wiki/Super_Stack!>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: SuperStackBang => "Super Stack!",
    versions: [SuperStackBang => "Super Stack!"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::SuperStackBang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SuperStackBang,
            syntax: r"0 33 100 108 114 111 87 32 44 111 108 108 101 72
if outputascii fi",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"0 33 100 108 114 111 87 32 44 111 108 108 101 72
if outputascii fi",
            )],
        }],
    };
