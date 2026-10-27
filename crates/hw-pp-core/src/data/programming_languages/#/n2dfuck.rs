//! Hello-world snippet for `ProgrammingLanguageName::N2dfuck`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#2DFuck>
//! - <https://esolangs.org/wiki/2DFuck>

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: N2dfuck => "2DFuck",
    versions: [N2dfuck => "2DFuck"],
    extensions: [],
}

const N2DFUCK: &str = r".!.!..!.!....!..!..!.!.!.!.!..!.!..!...!..!.!..!...!..!.!....!..!.!.!..!....!.!......!.!.!.!.!...!.!..!.!....!.!...!..!.!..!..!.!..!...!..!..!.!....!.!....!.";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N2dfuck,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N2dfuck,
            syntax: N2DFUCK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    N2DFUCK,
                    [(
                        TokenKind::Plain,
                        r".!.!..!.!....!..!..!.!.!.!.!..!.!..!...!..!.!..!...!..!.!....!..!.!.!..!....!.!......!.!.!.!.!...!.!..!.!....!.!...!..!.!..!..!.!..!...!..!..!.!....!.!....!.",
                    )],
                )
            },
        }],
    };
