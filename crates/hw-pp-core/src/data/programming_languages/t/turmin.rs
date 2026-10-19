//! Hello-world snippet for `ProgrammingLanguageName::Turmin`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Turmin>
//! - <https://esolangs.org/wiki/Turmin>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Turmin => "Turmin",
    versions: [Turmin => "Turmin"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Turmin,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Turmin,
            syntax: r"sHrserslrslrsors,rs rsWrsorsrrslrsdrs!",
            tokenized_syntax: &[(TokenKind::Plain, r"sHrserslrslrsors,rs rsWrsorsrrslrsdrs!")],
        }],
    };
