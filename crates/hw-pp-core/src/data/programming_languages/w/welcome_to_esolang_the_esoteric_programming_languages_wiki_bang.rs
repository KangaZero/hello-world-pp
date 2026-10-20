//! Hello-world snippet for `ProgrammingLanguageName::WelcomeToEsolangTheEsotericProgrammingLanguagesWikiBang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Welcome_to_Esolang,_the_esoteric_programming_languages_wiki!>
//! - <https://esolangs.org/wiki/Welcome_to_Esolang,_the_esoteric_programming_languages_wiki!>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: WelcomeToEsolangTheEsotericProgrammingLanguagesWikiBang => "Welcome to Esolang, the esoteric programming languages wiki!",
    versions: [WelcomeToEsolangTheEsotericProgrammingLanguagesWikiBang => "Welcome to Esolang, the esoteric programming languages wiki!"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::WelcomeToEsolangTheEsotericProgrammingLanguagesWikiBang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label:
                ProgrammingLanguageDialect::WelcomeToEsolangTheEsotericProgrammingLanguagesWikiBang,
            syntax: r"Welcome",
            tokenized_syntax: &[(TokenKind::Plain, r"Welcome")],
        }],
    };
