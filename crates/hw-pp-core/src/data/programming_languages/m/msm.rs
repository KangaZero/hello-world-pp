//! Hello-world snippet for `ProgrammingLanguageName::Msm`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#MSM>
//! - <https://esolangs.org/wiki/MSM>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Msm => "MSM",
    versions: [Msm => "MSM"],
    extensions: [],
}

const MSM: &str = r"!dlrow ',olleH............";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Msm,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Msm,
            syntax: MSM,
            tokenized_syntax: &const {
                checked_token_lengths(MSM, [(TokenKind::Plain, r"!dlrow ',olleH............")])
            },
        }],
    };
