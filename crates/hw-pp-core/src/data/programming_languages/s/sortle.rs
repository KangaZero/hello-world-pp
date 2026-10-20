//! Hello-world snippet for `ProgrammingLanguageName::Sortle`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Sortle>
//! - <https://esolangs.org/wiki/Sortle>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Sortle => "Sortle",
    versions: [Sortle => "Sortle"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sortle,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sortle,
            syntax: r#"hello := "hello, " ".!" "" ? ~
world := "(.....),.!" "" ? ", world" ~"#,
            tokenized_syntax: &[(
                TokenKind::Plain,
                r#"hello := "hello, " ".!" "" ? ~
world := "(.....),.!" "" ? ", world" ~"#,
            )],
        }],
    };
