//! Hello-world snippet for `ProgrammingLanguageName::Madbrain`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Madbrain>
//! - <https://esolangs.org/wiki/Madbrain>

use crate::data::programming_languages::ProgrammingParadigm::{StackBased, TwoDimensional};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Madbrain => "Madbrain",
    versions: [Madbrain => "Madbrain"],
    extensions: [],
}

const MADBRAIN: &str = r"9 5 9 9 5 7 8 7 5 5 9 5 8
8 5 4 4 3 6 4 4 3 3 4 5 4
* * * * * * * * * * * * *
c 4 3 3 2 2 c 4 2 4 3 4 1
2 * * * * + 2 * * + * * +
0 1 c c 7 c 0 7 7 6 c c c
g + 2 2 + 2 g + + * 2 2 x
  c 0 0 3 0   c 3 c 0 0
  2 g g * g   2 * 2 g g
  0     c     0 c 0
  g     2     g 2 g
        0       0
        g       g";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Madbrain,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased, TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Madbrain,
            syntax: MADBRAIN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MADBRAIN,
                    [(
                        TokenKind::Plain,
                        r"9 5 9 9 5 7 8 7 5 5 9 5 8
8 5 4 4 3 6 4 4 3 3 4 5 4
* * * * * * * * * * * * *
c 4 3 3 2 2 c 4 2 4 3 4 1
2 * * * * + 2 * * + * * +
0 1 c c 7 c 0 7 7 6 c c c
g + 2 2 + 2 g + + * 2 2 x
  c 0 0 3 0   c 3 c 0 0
  2 g g * g   2 * 2 g g
  0     c     0 c 0
  g     2     g 2 g
        0       0
        g       g",
                    )],
                )
            },
        }],
    };
