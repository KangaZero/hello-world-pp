//! Hello-world snippet for `ProgrammingLanguageName::Unicat`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Unicat>
//! - <https://esolangs.org/wiki/Unicat>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Unicat => "Unicat",
    versions: [Unicat => "Unicat"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Unicat,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Unicat,
            syntax: r"😻😹😸🙀🙀😹😹😸🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😹😼😽🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😹😽😼🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😹😽😼🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😹😽😿🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😽😼🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😼😸🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😹😺😿🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😹😽😿🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😹😾😺🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😹😽😼🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😹😼😼🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😼😹🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😹😺🙀🙀😽😼😸🙀🙀🙀🙀",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"😻😹😸🙀🙀😹😹😸🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😹😼😽🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😹😽😼🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😹😽😼🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😹😽😿🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😽😼🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😼😸🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😹😺😿🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😹😽😿🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😹😾😺🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😹😽😼🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😹😼😼🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😼😹🙀🙀😽😼😸🙀🙀😻😹😸🙀🙀😹😺🙀🙀😽😼😸🙀🙀🙀🙀",
            )],
        }],
    };
