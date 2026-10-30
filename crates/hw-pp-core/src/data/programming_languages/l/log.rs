//! Hello-world snippet for `ProgrammingLanguageName::Log`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Log>
//! - <https://esolangs.org/wiki/Log>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Log => "Log",
    versions: [Log => "Log"],
    extensions: [],
}

const LOG: &str = r"(A)
((A))
(A)
(A)
((A))
(A)
(A)
(A)
(A)
((A))
((A))
(A)
(A)
((A))
(A)
((A))
(A)
((A))
((A))
(A)
((A))
((A))
(A)
(A)
(A)
((A))
((A))
(A)
((A))
((A))
(A)
(A)
(A)
((A))
((A))
(A)
((A))
((A))
((A))
((A))
(A)
(A)
((A))
(A)
(A)
(A)
(A)
(A)";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Log,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Log,
            syntax: LOG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LOG,
                    [(
                        TokenKind::Plain,
                        r"(A)
((A))
(A)
(A)
((A))
(A)
(A)
(A)
(A)
((A))
((A))
(A)
(A)
((A))
(A)
((A))
(A)
((A))
((A))
(A)
((A))
((A))
(A)
(A)
(A)
((A))
((A))
(A)
((A))
((A))
(A)
(A)
(A)
((A))
((A))
(A)
((A))
((A))
((A))
((A))
(A)
(A)
((A))
(A)
(A)
(A)
(A)
(A)",
                    )],
                )
            },
        }],
    };
