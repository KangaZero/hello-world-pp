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
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Cast => "Cast",
    versions: [Cast => "Cast"],
    extensions: [],
}

const CAST: &str = r"Main
| IO<_> -> Main<>
| Main<> -> IO<IO.0<IO.1<IO.0<IO.0<IO.1<IO.0<IO.0<IO.0<IO.0<IO.1<IO.1<IO.0<IO.0<IO.1<IO.0<IO.1<
IO.0<IO.1<IO.1<IO.0<IO.1<IO.1<IO.0<IO.0<IO.0<IO.1<IO.1<IO.0<IO.1<IO.1<IO.0<IO.0<IO.0<IO.1<IO.1<
IO.0<IO.1<IO.1<IO.1<IO.1<IO.0<IO.0<IO.1<IO.0<IO.1<IO.1<IO.0<IO.0<IO.0<IO.0<IO.1<IO.0<IO.0<IO.0<
IO.0<IO.0<IO.0<IO.1<IO.1<IO.1<IO.0<IO.1<IO.1<IO.1<IO.0<IO.1<IO.1<IO.0<IO.1<IO.1<IO.1<IO.1<IO.0<
IO.1<IO.1<IO.1<IO.0<IO.0<IO.1<IO.0<IO.0<IO.1<IO.1<IO.0<IO.1<IO.1<IO.0<IO.0<IO.0<IO.1<IO.1<IO.0<
IO.0<IO.1<IO.0<IO.0<IO.0<IO.0<IO.1<IO.0<IO.0<IO.0<IO.0<IO.1<IO.null<>>>>>>>>>>>>>>>>>>>>>>>>>>>
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cast,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cast,
            syntax: CAST,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CAST,
                    [(
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
                )
            },
        }],
    };
