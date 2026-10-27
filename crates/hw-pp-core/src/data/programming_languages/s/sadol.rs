//! Hello-world snippet for `ProgrammingLanguageName::Sadol`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#SADOL>
//! - <https://esolangs.org/wiki/SADOL>

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Sadol => "SADOL",
    versions: [Sadol => "SADOL"],
    extensions: [],
}

const SADOL: &str = concat!(r#"!",212"#, output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sadol,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sadol,
            syntax: SADOL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SADOL,
                    [(
                        TokenKind::Plain,
                        concat!(r#"!",212"#, output_for_hello_world!()),
                    )],
                )
            },
        }],
    };
