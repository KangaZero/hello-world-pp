//! Hello-world snippet for `ProgrammingLanguageName::Ptote`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#PTotE>
//! - <https://esolangs.org/wiki/PTotE>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Ptote => "PTotE",
    versions: [Ptote => "PTotE"],
    extensions: [],
}

const PTOTE: &str = r"Eu U Es Es No V Pt No Db Es Pa Cr";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ptote,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ptote,
            syntax: PTOTE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PTOTE,
                    [(TokenKind::Plain, r"Eu U Es Es No V Pt No Db Es Pa Cr")],
                )
            },
        }],
    };
