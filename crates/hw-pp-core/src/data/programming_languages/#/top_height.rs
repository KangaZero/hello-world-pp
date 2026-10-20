//! Hello-world snippet for `ProgrammingLanguageName::TopHeight`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#(top,_height)>
//! - <https://esolangs.org/wiki/(top,_height)>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: TopHeight => "(top, height)",
    versions: [TopHeight => "(top, height)"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::TopHeight,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::TopHeight,
            syntax: r"H
 e lloX8W orldA                                                         1
 \++++++4+++++                  1                                1      ,              21           12      1  1  1
 \\ *                           ,,          ,                   a1                     ,2           ,,      ,  ,  ,
 \\                                                              -                      /        -",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"H
 e lloX8W orldA                                                         1
 \++++++4+++++                  1                                1      ,              21           12      1  1  1
 \\ *                           ,,          ,                   a1                     ,2           ,,      ,  ,  ,
 \\                                                              -                      /        -",
            )],
        }],
    };
