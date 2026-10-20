//! Hello-world snippet for `ProgrammingLanguageName::Nyascript`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#NyaScript>
//! - <https://esolangs.org/wiki/NyaScript>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Nyascript => "NyaScript",
    versions: [Nyascript => "NyaScript"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nyascript,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Nyascript,
            syntax: r"NyA NyA NyA NyA NyA NyA NyA NyA NyA NyA NYa 
Nya NyA NyA NyA NyA NyA NyA NyA Nya NyA NyA 
NyA NyA NyA NyA NyA NyA NyA NyA Nya NyA NyA 
NyA NyA Nya NyA NyA NyA Nya NyA NyA NyA NyA 
NyA NyA NyA NyA NyA Nya NyA NyA NyA NyA NyA
NyA nyA nyA nyA nyA nyA nyA nYa nYA Nya NyA 
NyA NYA Nya NyA NYA NyA NyA NyA NyA NyA NyA 
NyA NYA NYA nyan NyA NyA NyA NYA Nya NyA NyA 
NyA NyA NYA Nya NyA NyA NYA Nya nYa nYa nYa 
NYA NYAN NyA NyA NyA NYA NyA NyA NyA NYA NYAN
NYA nYa nYa nYa nYa nYa nYa nYa nYa NYA nyA
NyA NYA",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"NyA NyA NyA NyA NyA NyA NyA NyA NyA NyA NYa 
Nya NyA NyA NyA NyA NyA NyA NyA Nya NyA NyA 
NyA NyA NyA NyA NyA NyA NyA NyA Nya NyA NyA 
NyA NyA Nya NyA NyA NyA Nya NyA NyA NyA NyA 
NyA NyA NyA NyA NyA Nya NyA NyA NyA NyA NyA
NyA nyA nyA nyA nyA nyA nyA nYa nYA Nya NyA 
NyA NYA Nya NyA NYA NyA NyA NyA NyA NyA NyA 
NyA NYA NYA nyan NyA NyA NyA NYA Nya NyA NyA 
NyA NyA NYA Nya NyA NyA NYA Nya nYa nYa nYa 
NYA NYAN NyA NyA NyA NYA NyA NyA NyA NYA NYAN
NYA nYa nYa nYa nYa nYa nYa nYa nYa NYA nyA
NyA NYA",
            )],
        }],
    };
