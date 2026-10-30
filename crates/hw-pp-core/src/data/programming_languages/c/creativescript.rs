//! Hello-world snippet for `ProgrammingLanguageName::Creativescript`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#CreativeScript>
//! - <https://esolangs.org/wiki/CreativeScript>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Creativescript => "CreativeScript",
    versions: [Creativescript => "CreativeScript"],
    extensions: [],
}

const CREATIVESCRIPT: &str = concat!(r"say! ", output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Creativescript,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Creativescript,
            syntax: CREATIVESCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CREATIVESCRIPT,
                    [(
                        TokenKind::Plain,
                        concat!(r"say! ", output_for_hello_world!()),
                    )],
                )
            },
        }],
    };
