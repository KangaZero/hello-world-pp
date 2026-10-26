//! Hello-world snippet for `ProgrammingLanguageName::Difficult`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Difficult>
//! - <https://esolangs.org/wiki/Difficult>

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, ObjectOriented, Reflective, StackBased,
};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Difficult => "Difficult",
    versions: [Difficult => "Difficult"],
    extensions: [],
}

const DIFFICULT: &str = concat!(
    r#"thing.console.print(""#,
    output_for_hello_world!(),
    r#"")?"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Difficult,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[ObjectOriented, Functional, Reflective, StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Difficult,
            syntax: DIFFICULT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DIFFICULT,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"thing.console.print(""#,
                            output_for_hello_world!(),
                            r#"")?"#
                        ),
                    )],
                )
            },
        }],
    };
