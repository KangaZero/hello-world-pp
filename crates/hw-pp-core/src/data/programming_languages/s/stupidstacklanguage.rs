//! Hello-world snippet for `ProgrammingLanguageName::Stupidstacklanguage`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#StupidStackLanguage>
//! - <https://esolangs.org/wiki/StupidStackLanguage>

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Stupidstacklanguage => "StupidStackLanguage",
    versions: [Stupidstacklanguage => "StupidStackLanguage"],
    extensions: [],
}

const STUPIDSTACKLANGUAGE: &str = r"avqmimcfdddfviiffvddfavdegmfavdmcwfwdddfvddfwdfwdddf";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Stupidstacklanguage,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Stupidstacklanguage,
            syntax: STUPIDSTACKLANGUAGE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    STUPIDSTACKLANGUAGE,
                    [(
                        TokenKind::Plain,
                        r"avqmimcfdddfviiffvddfavdegmfavdmcwfwdddfvddfwdfwdddf",
                    )],
                )
            },
        }],
    };
