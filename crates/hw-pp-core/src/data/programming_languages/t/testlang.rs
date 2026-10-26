//! Hello-world snippet for `ProgrammingLanguageName::Testlang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#TESTLANG>
//! - <https://esolangs.org/wiki/TESTLANG>

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
    language: Testlang => "TESTLANG",
    versions: [Testlang => "TESTLANG"],
    extensions: [],
}

const TESTLANG: &str = r"out Hello, out char 32 out world! end";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Testlang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Testlang,
            syntax: TESTLANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TESTLANG,
                    [(TokenKind::Plain, r"out Hello, out char 32 out world! end")],
                )
            },
        }],
    };
