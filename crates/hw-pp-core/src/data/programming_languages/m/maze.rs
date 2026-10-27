//! Hello-world snippet for `ProgrammingLanguageName::Maze`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Maze>
//! - <https://esolangs.org/wiki/Maze>

use crate::data::programming_languages::ProgrammingParadigm::{Concurrent, TwoDimensional};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Maze => "Maze",
    versions: [Maze => "Maze"],
    extensions: [],
}

const MAZE: &str = concat!(
    r#"##,##,##
##,^^,## //Car Starts
##,AA,## //Do AA to Car
##,>>,## //Print Car
##,(),## //Destroy Car
##,##,##

AA-> =""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Maze,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Concurrent, TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Maze,
            syntax: MAZE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MAZE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"##,##,##
##,^^,## //Car Starts
##,AA,## //Do AA to Car
##,>>,## //Print Car
##,(),## //Destroy Car
##,##,##

AA-> =""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };
