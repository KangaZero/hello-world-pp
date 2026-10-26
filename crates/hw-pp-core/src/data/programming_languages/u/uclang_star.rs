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
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: UclangStar => "UClang*",
    versions: [UclangStar => "UClang*"],
    extensions: [],
}

const UCLANG_STAR: &str = r"A0 48 A1 A0 65 A1 A0 6C A1 A0 6C A1 A0 6F A1 A0 20 A1 A0 77 A1 A0 6F A1
A0 72 A1 A0 6C A1 A0 64 A1 AA";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::UclangStar,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::UclangStar,
            syntax: UCLANG_STAR,
            tokenized_syntax: &const {
                checked_token_lengths(
                    UCLANG_STAR,
                    [(
                        TokenKind::Plain,
                        r"A0 48 A1 A0 65 A1 A0 6C A1 A0 6C A1 A0 6F A1 A0 20 A1 A0 77 A1 A0 6F A1
A0 72 A1 A0 6C A1 A0 64 A1 AA",
                    )],
                )
            },
        }],
    };
