//! Hello-world snippet for `ProgrammingLanguageName::Cast`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Cast>
//! - <https://esolangs.org/wiki/Cast>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Cast => "Cast",
    versions: [Cast => "Cast"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cast,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cast,
            syntax: r"Main
| IO<_> -> Main<>
| Main<> -> IO<IO.0<IO.1<IO.0<IO.0<IO.1<IO.0<IO.0<IO.0<IO.0<IO.1<IO.1<IO.0<IO.0<IO.1<IO.0<IO.1<
IO.0<IO.1<IO.1<IO.0<IO.1<IO.1<IO.0<IO.0<IO.0<IO.1<IO.1<IO.0<IO.1<IO.1<IO.0<IO.0<IO.0<IO.1<IO.1<
IO.0<IO.1<IO.1<IO.1<IO.1<IO.0<IO.0<IO.1<IO.0<IO.1<IO.1<IO.0<IO.0<IO.0<IO.0<IO.1<IO.0<IO.0<IO.0<
IO.0<IO.0<IO.0<IO.1<IO.1<IO.1<IO.0<IO.1<IO.1<IO.1<IO.0<IO.1<IO.1<IO.0<IO.1<IO.1<IO.1<IO.1<IO.0<
IO.1<IO.1<IO.1<IO.0<IO.0<IO.1<IO.0<IO.0<IO.1<IO.1<IO.0<IO.1<IO.1<IO.0<IO.0<IO.0<IO.1<IO.1<IO.0<
IO.0<IO.1<IO.0<IO.0<IO.0<IO.0<IO.1<IO.0<IO.0<IO.0<IO.0<IO.1<IO.null<>>>>>>>>>>>>>>>>>>>>>>>>>>>
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"Main
| IO<_> -> Main<>
| Main<> -> IO<IO.0<IO.1<IO.0<IO.0<IO.1<IO.0<IO.0<IO.0<IO.0<IO.1<IO.1<IO.0<IO.0<IO.1<IO.0<IO.1<
IO.0<IO.1<IO.1<IO.0<IO.1<IO.1<IO.0<IO.0<IO.0<IO.1<IO.1<IO.0<IO.1<IO.1<IO.0<IO.0<IO.0<IO.1<IO.1<
IO.0<IO.1<IO.1<IO.1<IO.1<IO.0<IO.0<IO.1<IO.0<IO.1<IO.1<IO.0<IO.0<IO.0<IO.0<IO.1<IO.0<IO.0<IO.0<
IO.0<IO.0<IO.0<IO.1<IO.1<IO.1<IO.0<IO.1<IO.1<IO.1<IO.0<IO.1<IO.1<IO.0<IO.1<IO.1<IO.1<IO.1<IO.0<
IO.1<IO.1<IO.1<IO.0<IO.0<IO.1<IO.0<IO.0<IO.1<IO.1<IO.0<IO.1<IO.1<IO.0<IO.0<IO.0<IO.1<IO.1<IO.0<
IO.0<IO.1<IO.0<IO.0<IO.0<IO.0<IO.1<IO.0<IO.0<IO.0<IO.0<IO.1<IO.null<>>>>>>>>>>>>>>>>>>>>>>>>>>>
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>",
            )],
        }],
    };
