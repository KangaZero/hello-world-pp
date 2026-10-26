//! Hello-world snippet for `ProgrammingLanguageName::Xenbln`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#XENBLN>
//! - <https://esolangs.org/wiki/XENBLN>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Xenbln => "XENBLN",
    versions: [Xenbln => "XENBLN"],
    extensions: [],
}

const XENBLN: &str = r"š";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xenbln,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xenbln,
            syntax: XENBLN,
            tokenized_syntax: &const { checked_token_lengths(XENBLN, [(TokenKind::Plain, r"š")]) },
        }],
    };
