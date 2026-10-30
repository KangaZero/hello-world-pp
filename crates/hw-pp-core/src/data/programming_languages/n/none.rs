//! Hello-world snippet for `ProgrammingLanguageName::None`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#NONE>
//! - <https://esolangs.org/wiki/NONE>

use crate::data::programming_languages::ProgrammingParadigm::Imperative;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: None => "NONE",
    versions: [None => "NONE"],
    extensions: [],
}

const NONE: &str = r"+x----^pc+vp+v++++pp++++++p_+v++++++^p-v------p++++++p-v--pc+v--ps(++++)";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::None,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::None,
            syntax: NONE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NONE,
                    [(
                        TokenKind::Plain,
                        r"+x----^pc+vp+v++++pp++++++p_+v++++++^p-v------p++++++p-v--pc+v--ps(++++)",
                    )],
                )
            },
        }],
    };
