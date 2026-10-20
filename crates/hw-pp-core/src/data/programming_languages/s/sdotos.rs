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
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Sdotos => "SDOTOS",
    versions: [Sdotos => "SDOTOS"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sdotos,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sdotos,
            syntax: r"iHfptiefptilofpiiofpti,fpti fptiWfptiofptirfptilfptidfpti!fpt ",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"iHfptiefptilofpiiofpti,fpti fptiWfptiofptirfptilfptidfpti!fpt ",
            )],
        }],
    };
