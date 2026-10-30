//! Hello-world snippet for `ProgrammingLanguageName::Sashleyfuck`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Sashleyfuck>
//! - <https://esolangs.org/wiki/Sashleyfuck>

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Sashleyfuck => "Sashleyfuck",
    versions: [Sashleyfuck => "Sashleyfuck"],
    extensions: [],
}

const SASHLEYFUCK: &str = r"shhhhhhhh[ahhhhhhhhsl]ahhhhhhhheshhhhh[ahhhhhsl]ahhhhehhhhhhheehhheshhhhhhhhh[alllllllllsl]ahheshhhhhhh[ahhhhhhhsl]ahhhhhheshhhhh[ahhhhhsl]alehhhellllllelllllllleshhhhhhhh[allllllllsl]alllele";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sashleyfuck,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sashleyfuck,
            syntax: SASHLEYFUCK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SASHLEYFUCK,
                    [(
                        TokenKind::Plain,
                        r"shhhhhhhh[ahhhhhhhhsl]ahhhhhhhheshhhhh[ahhhhhsl]ahhhhehhhhhhheehhheshhhhhhhhh[alllllllllsl]ahheshhhhhhh[ahhhhhhhsl]ahhhhhheshhhhh[ahhhhhsl]alehhhellllllelllllllleshhhhhhhh[allllllllsl]alllele",
                    )],
                )
            },
        }],
    };
