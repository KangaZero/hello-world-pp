//! Hello-world snippet for `ProgrammingLanguageName::Integ`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Integ>
//! - <https://esolangs.org/wiki/Integ>

use crate::data::programming_languages::ProgrammingParadigm::Imperative;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Integ => "Integ",
    versions: [Integ => "Integ"],
    extensions: [],
}

const INTEG: &str = r"](104)](101)](108)](108)](111)](44)](32)](119)](111)](114)](108)](100)](10)";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Integ,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Integ,
            syntax: INTEG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    INTEG,
                    [(
                        TokenKind::Plain,
                        r"](104)](101)](108)](108)](111)](44)](32)](119)](111)](114)](108)](100)](10)",
                    )],
                )
            },
        }],
    };
