//! Hello-world snippet for `ProgrammingLanguageName::Funciton`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Funciton>
//! - <https://esolangs.org/wiki/Funciton>

use crate::data::programming_languages::ProgrammingParadigm::Declarative;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Funciton => "Funciton",
    versions: [Funciton => "Funciton"],
    extensions: [],
}

const FUNCITON: &str = r"   ╔═══════════════╗
   ║ 2388215291394 ║
   ║ 5970717094395 ║
   ║ 9830070909071 ║
   ║ 3359207734160 ║
   ║ 6896177782105 ║
   ║ 7169552834632 ║
   ╚═══════╤═══════╝
           │";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Funciton,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Declarative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Funciton,
            syntax: FUNCITON,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FUNCITON,
                    [(
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
                )
            },
        }],
    };
