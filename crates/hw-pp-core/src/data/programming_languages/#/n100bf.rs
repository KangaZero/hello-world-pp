//! Hello-world snippet for `ProgrammingLanguageName::N100bf`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#100BF>
//! - <https://esolangs.org/wiki/100BF>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: N100bf => "100BF",
    versions: [N100bf => "100BF"],
    extensions: [],
}

const N100BF: &str = r"100 100 100 100 100 100 100 100 lO0 1OO 100 100 100 100 lO0 1OO 100 100 1OO 100 100 100 1OO 100 100 100 1OO 100 1O0 1O0 1O0 1O0 10O lOO 1OO 100 1OO 100 1OO 10O 1OO 1OO 100 lO0 1O0 lOO 1O0 10O lOO 1OO 1OO l0O 1OO 10O 10O 10O l0O 100 100 100 100 100 100 100 l0O l0O 100 100 100 l0O 1OO 1OO l0O 1O0 10O l0O 1O0 l0O 100 100 100 l0O 10O 10O 10O 10O 10O 10O l0O 10O 10O 10O 10O 10O 10O 10O 10O l0O 1OO 1OO 100 l0O 1OO 100 100 l0O";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N100bf,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N100bf,
            syntax: N100BF,
            tokenized_syntax: &const {
                checked_token_lengths(
                    N100BF,
                    [(
                        TokenKind::Plain,
                        r"100 100 100 100 100 100 100 100 lO0 1OO 100 100 100 100 lO0 1OO 100 100 1OO 100 100 100 1OO 100 100 100 1OO 100 1O0 1O0 1O0 1O0 10O lOO 1OO 100 1OO 100 1OO 10O 1OO 1OO 100 lO0 1O0 lOO 1O0 10O lOO 1OO 1OO l0O 1OO 10O 10O 10O l0O 100 100 100 100 100 100 100 l0O l0O 100 100 100 l0O 1OO 1OO l0O 1O0 10O l0O 1O0 l0O 100 100 100 l0O 10O 10O 10O 10O 10O 10O l0O 10O 10O 10O 10O 10O 10O 10O 10O l0O 1OO 1OO 100 l0O 1OO 100 100 l0O",
                    )],
                )
            },
        }],
    };
