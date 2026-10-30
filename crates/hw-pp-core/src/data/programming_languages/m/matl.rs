//! Hello-world snippet for `ProgrammingLanguageName::Matl`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#MATL>
//! - <https://esolangs.org/wiki/MATL>

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Matl => "MATL",
    versions: [Matl => "MATL"],
    extensions: [],
}

const MATL: &str = concat!(r"'", output_for_hello_world!(), r"'");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Matl,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Matl,
            syntax: MATL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MATL,
                    [(
                        TokenKind::Plain,
                        concat!(r"'", output_for_hello_world!(), r"'"),
                    )],
                )
            },
        }],
    };
