//! Hello-world snippet for `ProgrammingLanguageName::QueueBasedEsolang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#queue-based_esolang>
//! - <https://esolangs.org/wiki/Queue-based_esolang>

use crate::data::programming_languages::ProgrammingParadigm::QueueBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: QueueBasedEsolang => "queue-based esolang",
    versions: [QueueBasedEsolang => "queue-based esolang"],
    extensions: [],
}

const QUEUE_BASED_ESOLANG: &str = r"pHpeplplpop,p pWpoprplpdp!QQQQQQQQQQQQQ";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::QueueBasedEsolang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[QueueBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::QueueBasedEsolang,
            syntax: QUEUE_BASED_ESOLANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    QUEUE_BASED_ESOLANG,
                    [(TokenKind::Plain, r"pHpeplplpop,p pWpoprplpdp!QQQQQQQQQQQQQ")],
                )
            },
        }],
    };
