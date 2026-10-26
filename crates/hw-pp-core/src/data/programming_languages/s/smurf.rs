//! Hello-world snippet for `ProgrammingLanguageName::Smurf`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Smurf>
//! - <https://esolangs.org/wiki/Smurf>

use crate::data::programming_languages::ProgrammingParadigm::{Reflective, StackBased};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Smurf => "Smurf",
    versions: [Smurf => "Smurf"],
    extensions: [],
}

const SMURF: &str = concat!(r#"""#, output_for_hello_world!(), r#""o"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Smurf,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Reflective, StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Smurf,
            syntax: SMURF,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SMURF,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#""o"#),
                    )],
                )
            },
        }],
    };
