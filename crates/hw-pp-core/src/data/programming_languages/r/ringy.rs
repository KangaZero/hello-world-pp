//! Hello-world snippet for `ProgrammingLanguageName::Ringy`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#RinGy>
//! - <https://esolangs.org/wiki/RinGy>

use crate::data::programming_languages::ProgrammingParadigm::Reflective;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Ringy => "RinGy",
    versions: [Ringy => "RinGy"],
    extensions: [],
}

const RINGY: &str = r"'H.'e.'l..'o.',.'!-.'w.'o.'r.'l.'d.'!._++++++++++.q";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ringy,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Reflective],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ringy,
            syntax: RINGY,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RINGY,
                    [(
                        TokenKind::Plain,
                        r"'H.'e.'l..'o.',.'!-.'w.'o.'r.'l.'d.'!._++++++++++.q",
                    )],
                )
            },
        }],
    };
