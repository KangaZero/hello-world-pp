//! Hello-world snippet for `ProgrammingLanguageName::ShindaSakana`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Shinda_Sakana>
//! - <https://esolangs.org/wiki/Shinda_Sakana>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: ShindaSakana => "Shinda Sakana",
    versions: [ShindaSakana => "Shinda Sakana"],
    extensions: [],
}

const SHINDA_SAKANA: &str = r"増増増乗減乗増増増増増増増増書増増増増増増増増増増増増増増増増増増増増増増増増増増増増増書増増増増増増増書書
増増増書減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減
減減減減減減減減減減減減減減減減減書減減減減減減減減減減減減書減減減減減減減減減減減減減減減減減減減減減乗減
減書減減減減減減減減書増増増書減減減減減減書減減減減減減減減書減減減減減減減減減減減減減減減減減減減減減減減
減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減書";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ShindaSakana,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ShindaSakana,
            syntax: SHINDA_SAKANA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SHINDA_SAKANA,
                    [(
                        TokenKind::Plain,
                        r"増増増乗減乗増増増増増増増増書増増増増増増増増増増増増増増増増増増増増増増増増増増増増増書増増増増増増増書書
増増増書減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減
減減減減減減減減減減減減減減減減減書減減減減減減減減減減減減書減減減減減減減減減減減減減減減減減減減減減乗減
減書減減減減減減減減書増増増書減減減減減減書減減減減減減減減書減減減減減減減減減減減減減減減減減減減減減減減
減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減減書",
                    )],
                )
            },
        }],
    };
