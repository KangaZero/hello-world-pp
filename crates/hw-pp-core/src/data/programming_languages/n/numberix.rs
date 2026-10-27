//! Hello-world snippet for `ProgrammingLanguageName::Numberix`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Numberix>
//! - <https://esolangs.org/wiki/Numberix>

use crate::data::programming_languages::ProgrammingParadigm::{CellBased, TwoDimensional};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Numberix => "Numberix",
    versions: [Numberix => "Numberix"],
    extensions: [],
}

const NUMBERIX: &str = r"A0000159006CA9006C590057A9006F590064A90021000000000000000000000000000000000000
59004809006559006F09002059007209006CFF0000";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Numberix,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased, TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Numberix,
            syntax: NUMBERIX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NUMBERIX,
                    [(
                        TokenKind::Plain,
                        r"A0000159006CA9006C590057A9006F590064A90021000000000000000000000000000000000000
59004809006559006F09002059007209006CFF0000",
                    )],
                )
            },
        }],
    };
