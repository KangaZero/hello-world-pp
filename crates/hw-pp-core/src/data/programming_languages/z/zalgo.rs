//! Hello-world snippet for `ProgrammingLanguageName::Zalgo`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Zalgo>
//! - <https://esolangs.org/wiki/Zalgo>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Zalgo => "Zalgo",
    versions: [Zalgo => "Zalgo"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zalgo,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zalgo,
            syntax: r"A̝̝̝̝̝̝̝̝̝̝̝̝̝̐̈̄̐̅̆̐̌̆̐̌̆̐̏̆̐̌̂̐̀̂̐̇̇̐̏̆̐̂̇̐̌̆̐̄̆̐́̂",
            tokenized_syntax: &[(TokenKind::Plain, r"A̝̝̝̝̝̝̝̝̝̝̝̝̝̐̈̄̐̅̆̐̌̆̐̌̆̐̏̆̐̌̂̐̀̂̐̇̇̐̏̆̐̂̇̐̌̆̐̄̆̐́̂")],
        }],
    };
