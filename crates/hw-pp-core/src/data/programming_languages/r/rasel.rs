//! Hello-world snippet for `ProgrammingLanguageName::Rasel`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#RASEL>
//! - <https://esolangs.org/wiki/RASEL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Rasel => "RASEL",
    versions: [Rasel => "RASEL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rasel,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rasel,
            syntax: r#"A"!dlroW ,olleH">:?@,Hj"#,
            tokenized_syntax: &[(TokenKind::Plain, r#"A"!dlroW ,olleH">:?@,Hj"#)],
        }],
    };
