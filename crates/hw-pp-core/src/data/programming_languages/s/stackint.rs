//! Hello-world snippet for `ProgrammingLanguageName::Stackint`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Stackint>
//! - <https://esolangs.org/wiki/Stackint>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Stackint => "Stackint",
    versions: [Stackint => "Stackint"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Stackint,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Stackint,
            syntax: r"98*99*54*+99*93*+99*93*+99*65*+84*99*6+99*65*+99*93*+6+99*93*+91+91+*",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"98*99*54*+99*93*+99*93*+99*65*+84*99*6+99*65*+99*93*+6+99*93*+91+91+*",
            )],
        }],
    };
