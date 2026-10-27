//! Hello-world snippet for `ProgrammingLanguageName::Sortle`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Sortle>
//! - <https://esolangs.org/wiki/Sortle>

use crate::data::programming_languages::ProgrammingParadigm::StringRewriting;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Sortle => "Sortle",
    versions: [Sortle => "Sortle"],
    extensions: [],
}

const SORTLE: &str = r#"hello := "hello, " ".!" "" ? ~
world := "(.....),.!" "" ? ", world" ~"#;

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sortle,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StringRewriting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sortle,
            syntax: SORTLE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SORTLE,
                    [(
                        TokenKind::Plain,
                        r#"hello := "hello, " ".!" "" ? ~
world := "(.....),.!" "" ? ", world" ~"#,
                    )],
                )
            },
        }],
    };
