//! Hello-world snippet for `ProgrammingLanguageName::Stbf`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#STBF>
//! - <https://esolangs.org/wiki/STBF>

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Stbf => "STBF",
    versions: [Stbf => "STBF"],
    extensions: [],
}

const STBF: &str = concat!(r"phrase ", output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Stbf,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Stbf,
            syntax: STBF,
            tokenized_syntax: &const {
                checked_token_lengths(
                    STBF,
                    [(
                        TokenKind::Plain,
                        concat!(r"phrase ", output_for_hello_world!()),
                    )],
                )
            },
        }],
    };
