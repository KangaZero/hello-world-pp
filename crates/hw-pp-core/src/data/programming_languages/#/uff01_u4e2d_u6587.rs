//! Hello-world snippet for `ProgrammingLanguageName::Uff01U4e2dU6587`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#%EF%BC%81%E4%B8%AD%E6%96%87>
//! - <https://esolangs.org/wiki/%EF%BC%81%E4%B8%AD%E6%96%87>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Uff01U4e2dU6587 => "！中文",
    versions: [Uff01U4e2dU6587 => "！中文"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Uff01U4e2dU6587,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Uff01U4e2dU6587,
            syntax: concat!(r"输出“", output_for_hello_world!(), r"”。"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"输出“", output_for_hello_world!(), r"”。"),
            )],
        }],
    };
