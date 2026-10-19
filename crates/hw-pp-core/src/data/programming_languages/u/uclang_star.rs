//! Hello-world snippet for `ProgrammingLanguageName::UclangStar`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#UClang%2A>
//! - <https://esolangs.org/wiki/UClang%2A>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: UclangStar => "UClang*",
    versions: [UclangStar => "UClang*"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::UclangStar,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::UclangStar,
            syntax: r"A0 48 A1 A0 65 A1 A0 6C A1 A0 6C A1 A0 6F A1 A0 20 A1 A0 77 A1 A0 6F A1
A0 72 A1 A0 6C A1 A0 64 A1 AA",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"A0 48 A1 A0 65 A1 A0 6C A1 A0 6C A1 A0 6F A1 A0 20 A1 A0 77 A1 A0 6F A1
A0 72 A1 A0 6C A1 A0 64 A1 AA",
            )],
        }],
    };
