//! Hello-world snippet for `ProgrammingLanguageName::Reversefuck`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#ReverseFuck>
//! - <https://esolangs.org/wiki/ReverseFuck>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Reversefuck => "ReverseFuck",
    versions: [Reversefuck => "ReverseFuck"],
    extensions: [],
}

const REVERSEFUCK: &str = r"--------]<----]<--<---<---<->>>>+[<-<-<+<<-]>[>+[<<,<+++,-------,,---,<<,>+,>,---,++++++,++++++++,<<-,<--,";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Reversefuck,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Reversefuck,
            syntax: REVERSEFUCK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    REVERSEFUCK,
                    [(
                        TokenKind::Plain,
                        r"--------]<----]<--<---<---<->>>>+[<-<-<+<<-]>[>+[<<,<+++,-------,,---,<<,>+,>,---,++++++,++++++++,<<-,<--,",
                    )],
                )
            },
        }],
    };
