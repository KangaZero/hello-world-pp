//! Hello-world snippet for `ProgrammingLanguageName::Varsig`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Varsig>
//! - <https://esolangs.org/wiki/Varsig>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Varsig => "Varsig",
    versions: [Varsig => "Varsig"],
    extensions: [],
}

const VARSIG: &str = r"!72)!101)!108:))!111)!44)!32)!87)!111)!114)!108)!100)!33)#";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Varsig,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Varsig,
            syntax: VARSIG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VARSIG,
                    [(
                        TokenKind::Plain,
                        r"!72)!101)!108:))!111)!44)!32)!87)!111)!114)!108)!100)!33)#",
                    )],
                )
            },
        }],
    };
