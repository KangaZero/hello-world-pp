//! Hello-world snippet for `ProgrammingLanguageName::Grass`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Grass>
//! - <https://esolangs.org/wiki/Grass>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Grass => "Grass",
    versions: [Grass => "Grass"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Grass,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Grass,
            syntax: r"wv
wwWWwWWWwv
Wwwww
WWw
WWWw
WWWWw
WWWWWw
WWWWWWw
WWWWWWWw 
Wwwwwwwwwwwww
WWWWw
WWWWWWWw
WWWWWWWWWWWWWWw
WWWWWWWWWWWww
WWWWWWWWWWww
WWWWWWWWWWWWw
WWWWWWWWWWww
WWWWWWWWWWwwwwwww
WWWWWWWWWWWWWWw
WWWWWWWWWWWWWWWWw
WWWWWWWWWWWWWWWWWWWWWWw
WWWWWWWWWWWWWWWWWWWww
WWWWWWWWWWWWWWWWWWww
WWWWWWWWWWWWWWWWWWwwwww
WWWWWWWWWWWWWWWWWWWWWww
WWWWWWWWWWWWWWWWWWWWWWWw
WWWWWWWWWWWWWWWWWWWWWWWWWWwwwwwwwwwwwwwwwwwwwwwwwwwww
Wwwwwwwwwwww
WWwwwwwww
WWWwwwwwww
WWWWw
WWWWWwwwwwwww
WWWWWWwwwwwwwwwwwwwwwww
WWWWWWWwwwwwwwwwwwwwwwwwwwww
WWWWWWWWwwwwwwwwwwwwwwwww
WWWWWWWWWwwww
WWWWWWWWWWwwwwwwwwwww
WWWWWWWWWWWwwwwwww
WWWWWWWWWWWWwwwwwwwwwwwwwwwwww
WWWWWWWWWWWWWwwwwwwwwwwwwwwwwwwwwwwwwww",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"wv
wwWWwWWWwv
Wwwww
WWw
WWWw
WWWWw
WWWWWw
WWWWWWw
WWWWWWWw 
Wwwwwwwwwwwww
WWWWw
WWWWWWWw
WWWWWWWWWWWWWWw
WWWWWWWWWWWww
WWWWWWWWWWww
WWWWWWWWWWWWw
WWWWWWWWWWww
WWWWWWWWWWwwwwwww
WWWWWWWWWWWWWWw
WWWWWWWWWWWWWWWWw
WWWWWWWWWWWWWWWWWWWWWWw
WWWWWWWWWWWWWWWWWWWww
WWWWWWWWWWWWWWWWWWww
WWWWWWWWWWWWWWWWWWwwwww
WWWWWWWWWWWWWWWWWWWWWww
WWWWWWWWWWWWWWWWWWWWWWWw
WWWWWWWWWWWWWWWWWWWWWWWWWWwwwwwwwwwwwwwwwwwwwwwwwwwww
Wwwwwwwwwwww
WWwwwwwww
WWWwwwwwww
WWWWw
WWWWWwwwwwwww
WWWWWWwwwwwwwwwwwwwwwww
WWWWWWWwwwwwwwwwwwwwwwwwwwww
WWWWWWWWwwwwwwwwwwwwwwwww
WWWWWWWWWwwww
WWWWWWWWWWwwwwwwwwwww
WWWWWWWWWWWwwwwwww
WWWWWWWWWWWWwwwwwwwwwwwwwwwwww
WWWWWWWWWWWWWwwwwwwwwwwwwwwwwwwwwwwwwww",
            )],
        }],
    };
