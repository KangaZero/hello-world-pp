//! Hello-world snippet for `ProgrammingLanguageName::BrokenCalculator`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Broken_Calculator>
//! - <https://esolangs.org/wiki/Broken_Calculator>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: BrokenCalculator => "Broken Calculator",
    versions: [BrokenCalculator => "Broken Calculator"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::BrokenCalculator,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::BrokenCalculator,
            syntax: r"PCR 72
PCR 69
PCR 76
PCR 76
PCR 79
PCR 32
PCR 87
PCR 79
PCR 82
PCR 76
PCR 68",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"PCR 72
PCR 69
PCR 76
PCR 76
PCR 79
PCR 32
PCR 87
PCR 79
PCR 82
PCR 76
PCR 68",
            )],
        }],
    };
