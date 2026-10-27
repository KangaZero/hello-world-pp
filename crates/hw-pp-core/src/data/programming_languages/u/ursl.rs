//! Hello-world snippet for `ProgrammingLanguageName::Ursl`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#URSL>
//! - <https://esolangs.org/wiki/URSL>

use crate::data::programming_languages::ProgrammingParadigm::{CellBased, QueueBased, StackBased};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Ursl => "URSL",
    versions: [Ursl => "URSL"],
    extensions: [],
}

const URSL: &str = r"v072-w0v0101-w0v0108-w0w0v0111-w0v044-w0v032-w0v0119-w0v0111-w0v0114-w0v0108-w0v0100-w0v033-w0";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ursl,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased, CellBased, QueueBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ursl,
            syntax: URSL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    URSL,
                    [(
                        TokenKind::Plain,
                        r"v072-w0v0101-w0v0108-w0w0v0111-w0v044-w0v032-w0v0119-w0v0111-w0v0114-w0v0108-w0v0100-w0v033-w0",
                    )],
                )
            },
        }],
    };
