//! Hello-world snippet for `ProgrammingLanguageName::Quipu`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Quipu>
//! - <https://esolangs.org/wiki/Quipu>

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
    language: Quipu => "Quipu",
    versions: [Quipu => "Quipu"],
    extensions: [],
}

const QUIPU: &str = r"'H
'e
'l
'l
'o
' 
'W
'o
'r
'l
'd
'!
\n
/\";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Quipu,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Quipu,
            syntax: QUIPU,
            tokenized_syntax: &const {
                checked_token_lengths(
                    QUIPU,
                    [(
                        TokenKind::Plain,
                        r"'H
'e
'l
'l
'o
' 
'W
'o
'r
'l
'd
'!
\n
/\",
                    )],
                )
            },
        }],
    };
