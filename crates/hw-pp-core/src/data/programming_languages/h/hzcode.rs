//! Hello-world snippet for `ProgrammingLanguageName::Hzcode`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#HZCode>
//! - <https://esolangs.org/wiki/HZCode>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Hzcode => "HZCode",
    versions: [Hzcode => "HZCode"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hzcode,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hzcode,
            syntax: r"十十十十十十十一一加加加加加加加加写百一加写百一一一一一一一一加加加加加加加加写百一一一一一一一一加加加加加加加加写百 十一加加写十十十十一一一一加加加加加加加写十十十一一加加加加写十十十十十十十十一一一一一一一加加加加加加加加加加加加加加写百十一加加写百十一一一一加加加加加写百一一一一一一一一加加加加加加加加写百写十十十一一一加加加加加写停",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"十十十十十十十一一加加加加加加加加写百一加写百一一一一一一一一加加加加加加加加写百一一一一一一一一加加加加加加加加写百 十一加加写十十十十一一一一加加加加加加加写十十十一一加加加加写十十十十十十十十一一一一一一一加加加加加加加加加加加加加加写百十一加加写百十一一一一加加加加加写百一一一一一一一一加加加加加加加加写百写十十十一一一加加加加加写停",
            )],
        }],
    };
