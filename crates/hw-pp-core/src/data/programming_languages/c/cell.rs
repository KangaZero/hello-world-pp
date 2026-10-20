//! Hello-world snippet for `ProgrammingLanguageName::Cell`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Cell>
//! - <https://esolangs.org/wiki/Cell>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Cell => "Cell",
    versions: [Cell => "Cell"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cell,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cell,
            syntax: r"j0 a72 . a101 . a108 . . a111 . a44 . a32 . a87 . a111 . a114 . a108 . a100 . a33 .",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"j0 a72 . a101 . a108 . . a111 . a44 . a32 . a87 . a111 . a114 . a108 . a100 . a33 .",
            )],
        }],
    };
