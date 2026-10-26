//! Hello-world snippet for `ProgrammingLanguageName::Reversible2d`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Reversible-2D>
//! - <https://esolangs.org/wiki/Reversible-2D>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Reversible2d => "Reversible-2D",
    versions: [Reversible2d => "Reversible-2D"],
    extensions: [],
}

const REVERSIBLE2D: &str = r" >PHOPeOPlOPlOPoOP,OP OPWOPoOPrOPlOPdOP!O¤";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Reversible2d,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Reversible2d,
            syntax: REVERSIBLE2D,
            tokenized_syntax: &const {
                checked_token_lengths(
                    REVERSIBLE2D,
                    [(
                        TokenKind::Plain,
                        r" >PHOPeOPlOPlOPoOP,OP OPWOPoOPrOPlOPdOP!O¤",
                    )],
                )
            },
        }],
    };
