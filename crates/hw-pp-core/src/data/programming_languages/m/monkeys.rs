//! Hello-world snippet for `ProgrammingLanguageName::Monkeys`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Monkeys>
//! - <https://esolangs.org/wiki/Monkeys>

use crate::data::programming_languages::ProgrammingParadigm::{ObjectOriented, TwoDimensional};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Monkeys => "Monkeys",
    versions: [Monkeys => "Monkeys"],
    extensions: [],
}

const MONKEYS: &str = r"1 RIGHT
1 RIGHT
1 RIGHT
1 RIGHT
1 RIGHT
1 RIGHT
1 DOWN
1 DOWN
1 LEFT
1 LEFT
2 LEFT
2 UP
2 RIGHT
2 LEFT
2 DOWN
2 UP
2 RIGHT
2 LEFT
2 DOWN
2 RIGHT
2 BOND
1 YELL
1 LEFT
1 LEFT
1 UP
1 UP
1 LEFT
1 LEFT
1 UP
1 UP
1 LEFT
1 DOWN
1 DOWN
1 LEFT
1 UP
1 UP
1 LEFT
1 DOWN
1 DOWN
1 RIGHT
1 DOWN
1 LEFT
1 TEACH
1 DOWN
1 RIGHT
1 UP
1 UP
1 UP
1 RIGHT
1 DOWN
1 DOWN
1 DOWN
1 DOWN
1 UP
1 RIGHT
1 RIGHT
1 DOWN
1 DOWN
1 TEACH
4 YELL
1 RIGHT
1 RIGHT
1 DOWN
1 DOWN
1 DOWN
1 LEFT
1 YELL
1 YELL
1 LEFT
1 LEFT
1 LEFT
2 LEFT
2 DOWN
2 RIGHT
2 UP
2 UP
2 LEFT
2 LEFT
2 LEFT
2 DOWN
2 DOWN
2 DOWN
2 RIGHT
2 RIGHT
2 RIGHT
2 RIGHT
2 UP 
2 UP
2 UP
2 LEFT
2 LEFT
2 DOWN
2 RIGHT
2 YELL
3 YELL
1 YELL
1 RIGHT
1 RIGHT
1 RIGHT
1 YELL
4 DOWN
4 TEACH
4 DOWN
4 DOWN
4 DOWN
4 DOWN
4 DOWN
2 RIGHT
6 LEFT
6 DOWN
6 RIGHT
6 UP
6 UP
6 LEFT
6 DOWN
6 RIGHT
6 YELL
4 YELL
2 YELL";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Monkeys,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[ObjectOriented, TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Monkeys,
            syntax: MONKEYS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MONKEYS,
                    [(
                        TokenKind::Plain,
                        r"1 RIGHT
1 RIGHT
1 RIGHT
1 RIGHT
1 RIGHT
1 RIGHT
1 DOWN
1 DOWN
1 LEFT
1 LEFT
2 LEFT
2 UP
2 RIGHT
2 LEFT
2 DOWN
2 UP
2 RIGHT
2 LEFT
2 DOWN
2 RIGHT
2 BOND
1 YELL
1 LEFT
1 LEFT
1 UP
1 UP
1 LEFT
1 LEFT
1 UP
1 UP
1 LEFT
1 DOWN
1 DOWN
1 LEFT
1 UP
1 UP
1 LEFT
1 DOWN
1 DOWN
1 RIGHT
1 DOWN
1 LEFT
1 TEACH
1 DOWN
1 RIGHT
1 UP
1 UP
1 UP
1 RIGHT
1 DOWN
1 DOWN
1 DOWN
1 DOWN
1 UP
1 RIGHT
1 RIGHT
1 DOWN
1 DOWN
1 TEACH
4 YELL
1 RIGHT
1 RIGHT
1 DOWN
1 DOWN
1 DOWN
1 LEFT
1 YELL
1 YELL
1 LEFT
1 LEFT
1 LEFT
2 LEFT
2 DOWN
2 RIGHT
2 UP
2 UP
2 LEFT
2 LEFT
2 LEFT
2 DOWN
2 DOWN
2 DOWN
2 RIGHT
2 RIGHT
2 RIGHT
2 RIGHT
2 UP 
2 UP
2 UP
2 LEFT
2 LEFT
2 DOWN
2 RIGHT
2 YELL
3 YELL
1 YELL
1 RIGHT
1 RIGHT
1 RIGHT
1 YELL
4 DOWN
4 TEACH
4 DOWN
4 DOWN
4 DOWN
4 DOWN
4 DOWN
2 RIGHT
6 LEFT
6 DOWN
6 RIGHT
6 UP
6 UP
6 LEFT
6 DOWN
6 RIGHT
6 YELL
4 YELL
2 YELL",
                    )],
                )
            },
        }],
    };
