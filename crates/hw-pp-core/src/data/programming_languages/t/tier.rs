//! Hello-world snippet for `ProgrammingLanguageName::Tier`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Tier>
//! - <https://esolangs.org/wiki/Tier>

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, StackBased};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Tier => "Tier",
    versions: [Tier => "Tier"],
    extensions: [],
}

const TIER: &str = concat!(r#"""#, output_for_hello_world!(), r#""{#"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tier,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative, StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tier,
            syntax: TIER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TIER,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#""{#"#),
                    )],
                )
            },
        }],
    };
