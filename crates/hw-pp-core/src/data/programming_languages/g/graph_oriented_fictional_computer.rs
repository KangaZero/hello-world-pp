//! Hello-world snippet for `ProgrammingLanguageName::GraphOrientedFictionalComputer`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Graph_Oriented_Fictional_Computer>
//! - <https://esolangs.org/wiki/Graph_Oriented_Fictional_Computer>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: GraphOrientedFictionalComputer => "Graph Oriented Fictional Computer",
    versions: [GraphOrientedFictionalComputer => "Graph Oriented Fictional Computer"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::GraphOrientedFictionalComputer,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::GraphOrientedFictionalComputer,
            syntax: concat!(
                r"SET_STRING A ",
                output_for_hello_world!(),
                r"
GET_STRING A"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"SET_STRING A ",
                    output_for_hello_world!(),
                    r"
GET_STRING A"
                ),
            )],
        }],
    };
