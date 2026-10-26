//! Hello-world snippet for `ProgrammingLanguageName::Bur`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Bur>
//! - <https://esolangs.org/wiki/Bur>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Bur => "Bur",
    versions: [Bur => "Bur"],
    extensions: [],
}

const BUR: &str =
    r"~@,#33!#100!#108!#114!#111!#87!#32!#44!#111!#108!#108!#101!#72!!.!.!.!.!.!.!.!.!.!.!.!.!v.;";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bur,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bur,
            syntax: BUR,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BUR,
                    [(
                        TokenKind::Plain,
                        r"~@,#33!#100!#108!#114!#111!#87!#32!#44!#111!#108!#108!#101!#72!!.!.!.!.!.!.!.!.!.!.!.!.!v.;",
                    )],
                )
            },
        }],
    };
