//! Hello-world snippet for `ProgrammingLanguageName::Free2edit`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Free2Edit>
//! - <https://esolangs.org/wiki/Free2Edit>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Free2edit => "Free2Edit",
    versions: [Free2edit => "Free2Edit"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Free2edit,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Free2edit,
            syntax: r"pHpeplplpop,p pWpoprplpdp!+5+5oc",
            tokenized_syntax: &[(TokenKind::Plain, r"pHpeplplpop,p pWpoprplpdp!+5+5oc")],
        }],
    };
