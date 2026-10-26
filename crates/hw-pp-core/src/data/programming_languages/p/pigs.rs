//! Hello-world snippet for `ProgrammingLanguageName::Pigs`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#pigs>
//! - <https://esolangs.org/wiki/Pigs>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Pigs => "pigs",
    versions: [Pigs => "pigs"],
    extensions: [],
}

const PIGS: &str = r"ppppppppiiiiiiiisgpppppppppppiiiiiiiiisg
pppppppppppiisgpppppppppppiisgppppppppppppiiiiiiiiisg
ppppiiiiiiiisgpppppppppiiisgppppppppppppiiiiiiiiisg
ppppppppppppiiiiiisgpppppppppppiisgppppppppppsg
ppppiiiiiiis";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pigs,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pigs,
            syntax: PIGS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PIGS,
                    [(
                        TokenKind::Plain,
                        r"ppppppppiiiiiiiisgpppppppppppiiiiiiiiisg
pppppppppppiisgpppppppppppiisgppppppppppppiiiiiiiiisg
ppppiiiiiiiisgpppppppppiiisgppppppppppppiiiiiiiiisg
ppppppppppppiiiiiisgpppppppppppiisgppppppppppsg
ppppiiiiiiis",
                    )],
                )
            },
        }],
    };
