//! Hello-world snippet for `ProgrammingLanguageName::Chickenfoot`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Chickenfoot>
//! - <https://esolangs.org/wiki/Chickenfoot>

use crate::data::programming_languages::ProgrammingParadigm::{
    CellBased, Imperative, TwoDimensional,
};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Chickenfoot => "Chickenfoot",
    versions: [Chickenfoot => "Chickenfoot"],
    extensions: [],
}

const CHICKENFOOT: &str = r"      ⠮
⠿⠈⠈⠈⠈⠠⠮⠫
     ⠰⠹   ⠮
      ⠈⠈⠈⠠⠮⠫
         ⠰⠹     ⠮
          ⠈⠈⠈⠈⠈⠠⠮⠫
               ⠰⠹     ⠮
                ⠈⠈⠈⠈⠈⠠⠮⠫
                     ⠰⠹      ⠮
                      ⠈⠈⠈⠈⠈⠈⠠⠮⠫
                            ⠰⠹     ⠮
                            ⠈⠈⠈⠈⠈⠈⠠⠮⠫
                                  ⠰⠹⠮
 ⠮⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯
⠬        ⠮
⠈⠈⠈⠈⠈⠈⠈⠈⠠⠮⠫
        ⠰⠹      ⠮
         ⠈⠈⠈⠈⠈⠈⠠⠮⠫
               ⠰⠹       ⠮
                ⠈⠈⠈⠈⠈⠈⠈⠠⠮⠫
                       ⠰⠹     ⠮
                        ⠈⠈⠈⠈⠈⠠⠮⠫
                             ⠰⠹
                              ⠈⠈⠠⠮⠫
                                ⠰⠹";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Chickenfoot,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative, CellBased, TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Chickenfoot,
            syntax: CHICKENFOOT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CHICKENFOOT,
                    [(
                        TokenKind::Plain,
                        r"      ⠮
⠿⠈⠈⠈⠈⠠⠮⠫
     ⠰⠹   ⠮
      ⠈⠈⠈⠠⠮⠫
         ⠰⠹     ⠮
          ⠈⠈⠈⠈⠈⠠⠮⠫
               ⠰⠹     ⠮
                ⠈⠈⠈⠈⠈⠠⠮⠫
                     ⠰⠹      ⠮
                      ⠈⠈⠈⠈⠈⠈⠠⠮⠫
                            ⠰⠹     ⠮
                            ⠈⠈⠈⠈⠈⠈⠠⠮⠫
                                  ⠰⠹⠮
 ⠮⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯⠯
⠬        ⠮
⠈⠈⠈⠈⠈⠈⠈⠈⠠⠮⠫
        ⠰⠹      ⠮
         ⠈⠈⠈⠈⠈⠈⠠⠮⠫
               ⠰⠹       ⠮
                ⠈⠈⠈⠈⠈⠈⠈⠠⠮⠫
                       ⠰⠹     ⠮
                        ⠈⠈⠈⠈⠈⠠⠮⠫
                             ⠰⠹
                              ⠈⠈⠠⠮⠫
                                ⠰⠹",
                    )],
                )
            },
        }],
    };
