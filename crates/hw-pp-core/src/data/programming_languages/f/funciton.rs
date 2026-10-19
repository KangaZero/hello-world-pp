//! Hello-world snippet for `ProgrammingLanguageName::Funciton`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Funciton>
//! - <https://esolangs.org/wiki/Funciton>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Funciton => "Funciton",
    versions: [Funciton => "Funciton"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Funciton,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Funciton,
            syntax: r"   ╔═══════════════╗
   ║ 2388215291394 ║
   ║ 5970717094395 ║
   ║ 9830070909071 ║
   ║ 3359207734160 ║
   ║ 6896177782105 ║
   ║ 7169552834632 ║
   ╚═══════╤═══════╝
           │",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"   ╔═══════════════╗
   ║ 2388215291394 ║
   ║ 5970717094395 ║
   ║ 9830070909071 ║
   ║ 3359207734160 ║
   ║ 6896177782105 ║
   ║ 7169552834632 ║
   ╚═══════╤═══════╝
           │",
            )],
        }],
    };
