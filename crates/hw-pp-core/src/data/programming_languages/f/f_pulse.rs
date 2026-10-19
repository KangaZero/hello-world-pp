//! Hello-world snippet for `ProgrammingLanguageName::FPulse`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#F-PULSE>
//! - <https://esolangs.org/wiki/F-PULSE>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: FPulse => "F-PULSE",
    versions: [FPulse => "F-PULSE"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::FPulse,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::FPulse,
            syntax: r"PLS PLS PLS PLS PLS PLS PLS PLS PLS PLS CBGN[ NXT PLS PLS PLS PLS PLS PLS PLS NXT PLS PLS PLS PLS PLS PLS PLS PLS PLS PLS NXT PLS PLS PLS LST LST LST MNS ]CEND NXT PLS PLS OUT NXT PLS OUT PLS PLS PLS PLS PLS PLS PLS OUT OUT PLS PLS PLS OUT NXT PLS PLS OUT LST LST PLS PLS PLS PLS PLS PLS PLS PLS PLS PLS PLS PLS PLS PLS PLS OUT NXT OUT PLS PLS PLS OUT MNS MNS MNS MNS MNS MNS OUT MNS MNS MNS MNS MNS MNS MNS MNS OUT NXT PLS OUT NXT PTN OUT",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"PLS PLS PLS PLS PLS PLS PLS PLS PLS PLS CBGN[ NXT PLS PLS PLS PLS PLS PLS PLS NXT PLS PLS PLS PLS PLS PLS PLS PLS PLS PLS NXT PLS PLS PLS LST LST LST MNS ]CEND NXT PLS PLS OUT NXT PLS OUT PLS PLS PLS PLS PLS PLS PLS OUT OUT PLS PLS PLS OUT NXT PLS PLS OUT LST LST PLS PLS PLS PLS PLS PLS PLS PLS PLS PLS PLS PLS PLS PLS PLS OUT NXT OUT PLS PLS PLS OUT MNS MNS MNS MNS MNS MNS OUT MNS MNS MNS MNS MNS MNS MNS MNS OUT NXT PLS OUT NXT PTN OUT",
            )],
        }],
    };
