//! Hello-world snippet for `ProgrammingLanguageName::Enscript`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#EnScript>
//! - <https://esolangs.org/wiki/EnScript>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Enscript => "EnScript",
    versions: [Enscript => "EnScript"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Enscript,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Enscript,
            syntax: r"LDA A, 72
OUT A
LDA A, 101
OUT A
LDA A, 108
OUT A
OUT A
LDA A, 111
OUT A
LDA A, 44
OUT A
LDA A, 32
OUT A
LDA A, 119
OUT A
LDA A, 111
OUT A
LDA A, 114
OUT A
LDA A, 108
OUT A
LDA A, 100
OUT A
LDA A, 33
OUT A
LDA A, 9",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"LDA A, 72
OUT A
LDA A, 101
OUT A
LDA A, 108
OUT A
OUT A
LDA A, 111
OUT A
LDA A, 44
OUT A
LDA A, 32
OUT A
LDA A, 119
OUT A
LDA A, 111
OUT A
LDA A, 114
OUT A
LDA A, 108
OUT A
LDA A, 100
OUT A
LDA A, 33
OUT A
LDA A, 9",
            )],
        }],
    };
