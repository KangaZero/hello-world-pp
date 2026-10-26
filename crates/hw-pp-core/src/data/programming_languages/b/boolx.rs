//! Hello-world snippet for `ProgrammingLanguageName::Boolx`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#BoolX>
//! - <https://esolangs.org/wiki/BoolX>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Boolx => "BoolX",
    versions: [Boolx => "BoolX"],
    extensions: [],
}

const BOOLX: &str = r"_+_+_+^+_+_+^]=
^+_+^+_+_+^+^]=
_+_+^+^+_+^+^]]=
^+^+^+^+_+^+^]=
_+_+^+^+_+^+_]=
_+_+_+_+_+^+_]=
^+^+^+_+^+^+^]=
^+^+^+^+_+^+^]=
_+^+_+_+^+^+^]=
_+_+^+^+_+^+^]=
_+_+^+_+_+^+^]=
^+_+_+_+_+^+_]=
_+^+_+^+*]";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Boolx,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Boolx,
            syntax: BOOLX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BOOLX,
                    [(
                        TokenKind::Plain,
                        r"_+_+_+^+_+_+^]=
^+_+^+_+_+^+^]=
_+_+^+^+_+^+^]]=
^+^+^+^+_+^+^]=
_+_+^+^+_+^+_]=
_+_+_+_+_+^+_]=
^+^+^+_+^+^+^]=
^+^+^+^+_+^+^]=
_+^+_+_+^+^+^]=
_+_+^+^+_+^+^]=
_+_+^+_+_+^+^]=
^+_+_+_+_+^+_]=
_+^+_+^+*]",
                    )],
                )
            },
        }],
    };
