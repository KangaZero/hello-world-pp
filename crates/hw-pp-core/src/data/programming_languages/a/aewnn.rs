//! Hello-world snippet for `ProgrammingLanguageName::Aewnn`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#AEWNN>
//! - <https://esolangs.org/wiki/AEWNN>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Aewnn => "AEWNN",
    versions: [Aewnn => "AEWNN"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Aewnn,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Aewnn,
            syntax: r"[8+]cpar2[5+]cpa[7+]cpacpa+++cpa [8+]cpar2[15+]cpa+++cpar2[12+]cpar2++++cpa",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"[8+]cpar2[5+]cpa[7+]cpacpa+++cpa [8+]cpar2[15+]cpa+++cpar2[12+]cpar2++++cpa",
            )],
        }],
    };
