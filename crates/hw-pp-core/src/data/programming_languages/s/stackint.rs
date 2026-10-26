//! Hello-world snippet for `ProgrammingLanguageName::Stackint`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Stackint>
//! - <https://esolangs.org/wiki/Stackint>

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
    language: Stackint => "Stackint",
    versions: [Stackint => "Stackint"],
    extensions: [],
}

const STACKINT: &str = r"98*99*54*+99*93*+99*93*+99*65*+84*99*6+99*65*+99*93*+6+99*93*+91+91+*";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Stackint,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Stackint,
            syntax: STACKINT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    STACKINT,
                    [(
                        TokenKind::Plain,
                        r"98*99*54*+99*93*+99*93*+99*65*+84*99*6+99*65*+99*93*+6+99*93*+91+91+*",
                    )],
                )
            },
        }],
    };
