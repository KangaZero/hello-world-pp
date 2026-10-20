//! Hello-world snippet for `ProgrammingLanguageName::IFuckYouFuck`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#I_fuck,_you_fuck>
//! - <https://esolangs.org/wiki/I_fuck,_you_fuck>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: IFuckYouFuck => "I fuck, you fuck",
    versions: [IFuckYouFuck => "I fuck, you fuck"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::IFuckYouFuck,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::IFuckYouFuck,
            syntax: r#"fuck "SGVsbG8sIFdvcmxkIQ==""#,
            tokenized_syntax: &[(TokenKind::Plain, r#"fuck "SGVsbG8sIFdvcmxkIQ==""#)],
        }],
    };
