//! Hello-world snippet for `ProgrammingLanguageName::HelloFuckBang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Hello_Fuck!>
//! - <https://esolangs.org/wiki/Hello_Fuck!>

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: HelloFuckBang => "Hello Fuck!",
    versions: [HelloFuckBang => "Hello Fuck!"],
    extensions: [],
}

const HELLO_FUCK_BANG: &str = r".*.";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::HelloFuckBang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::HelloFuckBang,
            syntax: HELLO_FUCK_BANG,
            tokenized_syntax: &const {
                checked_token_lengths(HELLO_FUCK_BANG, [(TokenKind::Plain, r".*.")])
            },
        }],
    };
