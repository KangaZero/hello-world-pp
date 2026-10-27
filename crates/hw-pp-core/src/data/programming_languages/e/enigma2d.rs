//! Hello-world snippet for `ProgrammingLanguageName::Enigma2d`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Enigma-2D>
//! - <https://esolangs.org/wiki/Enigma-2D>

use crate::data::programming_languages::ProgrammingParadigm::TwoDimensional;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Enigma2d => "Enigma-2D",
    versions: [Enigma2d => "Enigma-2D"],
    extensions: [],
}

const ENIGMA2D: &str = r"  >+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.>>>++++++++[<++++>-]D
                 .+>>.--------.------.+++.<<<<.---<]->+++++++++<[++++++++++>>>.<L";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Enigma2d,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Enigma2d,
            syntax: ENIGMA2D,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ENIGMA2D,
                    [(
                        TokenKind::Plain,
                        r"  >+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.>>>++++++++[<++++>-]D
                 .+>>.--------.------.+++.<<<<.---<]->+++++++++<[++++++++++>>>.<L",
                    )],
                )
            },
        }],
    };
