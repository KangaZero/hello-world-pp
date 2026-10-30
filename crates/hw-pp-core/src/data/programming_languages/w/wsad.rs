//! Hello-world snippet for `ProgrammingLanguageName::Wsad`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#wsad>
//! - <https://esolangs.org/wiki/Wsad>

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Wsad => "wsad",
    versions: [Wsad => "wsad"],
    extensions: [],
}

const WSAD: &str = r"wwwwwwwwwwwwwwwwwasdwwwwwwwwwasdwwwwsdwwwwwwsdwwwwwwsdwwsasasasawswdsdwwsdwwsdwssdsdwwwasawdsawswdsdsdsssdwswswssswwwwwwwwwwwwwwsssswwwwwwsssdsdsssawssssasswwwwwwsswswswswswswssswswswswswswswswssssdsdwwsssdwwwwss";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Wsad,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Wsad,
            syntax: WSAD,
            tokenized_syntax: &const {
                checked_token_lengths(
                    WSAD,
                    [(
                        TokenKind::Plain,
                        r"wwwwwwwwwwwwwwwwwasdwwwwwwwwwasdwwwwsdwwwwwwsdwwwwwwsdwwsasasasawswdsdwwsdwwsdwssdsdwwwasawdsawswdsdsdsssdwswswssswwwwwwwwwwwwwwsssswwwwwwsssdsdsssawssssasswwwwwwsswswswswswswssswswswswswswswswssssdsdwwsssdwwwwss",
                    )],
                )
            },
        }],
    };
