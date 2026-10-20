//! Hello-world snippet for `ProgrammingLanguageName::Stackfest`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Stackfest>
//! - <https://esolangs.org/wiki/Stackfest>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Stackfest => "Stackfest",
    versions: [Stackfest => "Stackfest"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Stackfest,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Stackfest,
            syntax: r#"st = >>
push "!dlroW ,olleH" into st
while st != null:
    outascii from st"#,
            tokenized_syntax: &[(
                TokenKind::Plain,
                r#"st = >>
push "!dlroW ,olleH" into st
while st != null:
    outascii from st"#,
            )],
        }],
    };
