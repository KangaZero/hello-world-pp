//! Hello-world snippet for `ProgrammingLanguageName::BitwiseCyclicTag`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Bitwise_Cyclic_Tag>
//! - <https://esolangs.org/wiki/Bitwise_Cyclic_Tag>

use crate::data::programming_languages::ProgrammingParadigm::QueueBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: BitwiseCyclicTag => "Bitwise Cyclic Tag",
    versions: [BitwiseCyclicTag => "Bitwise Cyclic Tag"],
    extensions: [],
}

const BITWISE_CYCLIC_TAG: &str = r"1";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::BitwiseCyclicTag,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[QueueBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::BitwiseCyclicTag,
            syntax: BITWISE_CYCLIC_TAG,
            tokenized_syntax: &const {
                checked_token_lengths(BITWISE_CYCLIC_TAG, [(TokenKind::Plain, r"1")])
            },
        }],
    };
