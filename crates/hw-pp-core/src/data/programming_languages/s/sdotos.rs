//! Hello-world snippet for `ProgrammingLanguageName::Sdotos`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#SDOTOS>
//! - <https://esolangs.org/wiki/SDOTOS>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Sdotos => "SDOTOS",
    versions: [Sdotos => "SDOTOS"],
    extensions: [],
}

const SDOTOS: &str = r"iHfptiefptilofpiiofpti,fpti fptiWfptiofptirfptilfptidfpti!fpt ";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sdotos,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sdotos,
            syntax: SDOTOS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SDOTOS,
                    [(
                        TokenKind::Plain,
                        r"iHfptiefptilofpiiofpti,fpti fptiWfptiofptirfptilfptidfpti!fpt ",
                    )],
                )
            },
        }],
    };
