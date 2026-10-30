//! Hello-world snippet for `ProgrammingLanguageName::Fishing`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Fishing>
//! - <https://esolangs.org/wiki/Fishing>

use crate::data::programming_languages::ProgrammingParadigm::{CellBased, TwoDimensional};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Fishing => "Fishing",
    versions: [Fishing => "Fishing"],
    extensions: [],
}

const FISHING: &str = concat!(
    r"v+CCCCCCCCCCCCCCCC
  `",
    output_for_hello_world!(),
    r"`N"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fishing,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased, TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fishing,
            syntax: FISHING,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FISHING,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"v+CCCCCCCCCCCCCCCC
  `",
                            output_for_hello_world!(),
                            r"`N"
                        ),
                    )],
                )
            },
        }],
    };
