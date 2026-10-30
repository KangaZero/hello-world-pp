//! Hello-world snippet for `ProgrammingLanguageName::StackBased`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Stack-based>
//! - <https://esolangs.org/wiki/Stack-based>

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Procedural, StackBased};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: StackBased => "Stack-based",
    versions: [StackBased => "Stack-based"],
    extensions: [],
}

const STACK_BASED: &str = concat!(r#"P ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::StackBased,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative, Procedural, StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::StackBased,
            syntax: STACK_BASED,
            tokenized_syntax: &const {
                checked_token_lengths(
                    STACK_BASED,
                    [(
                        TokenKind::Plain,
                        concat!(r#"P ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
