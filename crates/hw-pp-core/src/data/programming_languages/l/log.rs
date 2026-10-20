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
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Log => "Log",
    versions: [Log => "Log"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Log,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Log,
            syntax: r"(A)
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
            tokenized_syntax: &[(
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
        }],
    };
