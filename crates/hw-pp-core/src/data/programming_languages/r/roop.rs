//! Hello-world snippet for `ProgrammingLanguageName::Roop`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#ROOP>
//! - <https://esolangs.org/wiki/ROOP>
//! - <https://www.wikidata.org/wiki/Q7277404> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Logic, ObjectOriented, Procedural, TwoDimensional};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Roop => "ROOP",
    versions: [Roop => "ROOP"],
    extensions: [],
}

const ROOP: &str = concat!(
    r#"""#,
    output_for_hello_world!(),
    r#""
h"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Roop,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Procedural, ObjectOriented, Logic, TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Roop,
            syntax: ROOP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ROOP,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"""#,
                            output_for_hello_world!(),
                            r#""
h"#
                        ),
                    )],
                )
            },
        }],
    };
