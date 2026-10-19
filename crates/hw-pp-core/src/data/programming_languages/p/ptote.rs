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
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Ptote => "PTotE",
    versions: [Ptote => "PTotE"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ptote,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ptote,
            syntax: r"Eu U Es Es No V Pt No Db Es Pa Cr",
            tokenized_syntax: &[(TokenKind::Plain, r"Eu U Es Es No V Pt No Db Es Pa Cr")],
        }],
    };
