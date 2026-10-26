//! Hello-world snippet for `ProgrammingLanguageName::Moremathrpn`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#MoreMathRPN>
//! - <https://esolangs.org/wiki/MoreMathRPN>

use crate::data::programming_languages::ProgrammingParadigm::{Functional, Imperative, StackBased};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Moremathrpn => "MoreMathRPN",
    versions: [Moremathrpn => "MoreMathRPN"],
    extensions: [],
}

const MOREMATHRPN: &str = concat!(r#"outputS ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Moremathrpn,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative, Functional, StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Moremathrpn,
            syntax: MOREMATHRPN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MOREMATHRPN,
                    [(
                        TokenKind::Plain,
                        concat!(r#"outputS ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
