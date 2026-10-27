//! Hello-world snippet for `ProgrammingLanguageName::Myth`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Myth>
//! - <https://esolangs.org/wiki/Myth>

use crate::data::programming_languages::ProgrammingParadigm::StringRewriting;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Myth => "Myth",
    versions: [Myth => "Myth"],
    extensions: [],
}

const MYTH: &str = concat!(r#"{"_":""#, output_for_hello_world!(), r#""}"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Myth,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StringRewriting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Myth,
            syntax: MYTH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MYTH,
                    [(
                        TokenKind::Plain,
                        concat!(r#"{"_":""#, output_for_hello_world!(), r#""}"#),
                    )],
                )
            },
        }],
    };
