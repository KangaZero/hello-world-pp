//! Hello-world snippet for `ProgrammingLanguageName::Fishing`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Fishing>
//! - <https://esolangs.org/wiki/Fishing>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Fishing => "Fishing",
    versions: [Fishing => "Fishing"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fishing,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fishing,
            syntax: concat!(
                r"v+CCCCCCCCCCCCCCCC
  `",
                output_for_hello_world!(),
                r"`N"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"v+CCCCCCCCCCCCCCCC
  `",
                    output_for_hello_world!(),
                    r"`N"
                ),
            )],
        }],
    };
