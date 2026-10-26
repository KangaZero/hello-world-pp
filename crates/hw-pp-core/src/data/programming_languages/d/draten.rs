//! Hello-world snippet for `ProgrammingLanguageName::Draten`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Draten>
//! - <https://esolangs.org/wiki/Draten>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Draten => "Draten",
    versions: [Draten => "Draten"],
    extensions: [],
}

const DRATEN: &str = r"KKJJJJJIIIIIIIHHHHHHGGGGGGGG┓  
   ➧AAAABBBCCCCCDDDDDEEEEEE┓┃    ┏━━┓        ┏F┓
                           ┃┃    ┃┏┓┗┓  ┏┓  ┏┛┏┛
                           ┃┗━━━━┛┃┗┓┗┓┏┛┗┓┏┛┏┛
                           ┃┏━━━━┓┃ ┗┓┗┛┏┓┗┛┏┛
                           ┃┃    ┃┃  ┗┓┏┛┗┓┏┛
                           ┗┛    ┗┛   ┗┛  ┗┛";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Draten,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Draten,
            syntax: DRATEN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DRATEN,
                    [(
                        TokenKind::Plain,
                        r"KKJJJJJIIIIIIIHHHHHHGGGGGGGG┓  
   ➧AAAABBBCCCCCDDDDDEEEEEE┓┃    ┏━━┓        ┏F┓
                           ┃┃    ┃┏┓┗┓  ┏┓  ┏┛┏┛
                           ┃┗━━━━┛┃┗┓┗┓┏┛┗┓┏┛┏┛
                           ┃┏━━━━┓┃ ┗┓┗┛┏┓┗┛┏┛
                           ┃┃    ┃┃  ┗┓┏┛┗┓┏┛
                           ┗┛    ┗┛   ┗┛  ┗┛",
                    )],
                )
            },
        }],
    };
