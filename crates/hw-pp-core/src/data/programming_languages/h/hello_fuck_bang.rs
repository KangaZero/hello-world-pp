//! Hello-world snippet for `ProgrammingLanguageName::HelloFuckBang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Hello_Fuck!>
//! - <https://esolangs.org/wiki/Hello_Fuck!>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: HelloFuckBang => "Hello Fuck!",
    versions: [HelloFuckBang => "Hello Fuck!"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::HelloFuckBang,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::HelloFuckBang,
            syntax: r".*.",
            tokenized_syntax: &[(TokenKind::Plain, r".*.")],
        }],
    };
