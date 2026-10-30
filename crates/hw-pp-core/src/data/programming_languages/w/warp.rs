//! Hello-world snippet for `ProgrammingLanguageName::Warp`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#WARP>
//! - <https://esolangs.org/wiki/WARP>

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Warp => "WARP",
    versions: [Warp => "WARP"],
    extensions: [],
}

const WARP: &str = concat!(r#")""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Warp,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Warp,
            syntax: WARP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    WARP,
                    [(
                        TokenKind::Plain,
                        concat!(r#")""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
