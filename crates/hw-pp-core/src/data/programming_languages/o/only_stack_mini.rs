//! Hello-world snippet for `ProgrammingLanguageName::OnlyStackMini`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#ONLY_STACK_mini>
//! - <https://esolangs.org/wiki/ONLY_STACK_mini>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: OnlyStackMini => "ONLY_STACK mini",
    versions: [OnlyStackMini => "ONLY_STACK mini"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::OnlyStackMini,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::OnlyStackMini,
            syntax: r"#72 . #101 . #108 . #108 . #111 . #44 . #32 . #87 . #111 . #114 . #108 . #100 . #33 . !",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"#72 . #101 . #108 . #108 . #111 . #44 . #32 . #87 . #111 . #114 . #108 . #100 . #33 . !",
            )],
        }],
    };
