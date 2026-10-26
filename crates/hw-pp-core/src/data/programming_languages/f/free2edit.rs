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
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Free2edit => "Free2Edit",
    versions: [Free2edit => "Free2Edit"],
    extensions: [],
}

const FREE2EDIT: &str = r"pHpeplplpop,p pWpoprplpdp!+5+5oc";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Free2edit,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Free2edit,
            syntax: FREE2EDIT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FREE2EDIT,
                    [(TokenKind::Plain, r"pHpeplplpop,p pWpoprplpdp!+5+5oc")],
                )
            },
        }],
    };
