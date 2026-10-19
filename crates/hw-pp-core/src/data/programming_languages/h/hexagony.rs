//! Hello-world snippet for `ProgrammingLanguageName::Hexagony`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Hexagony>
//! - <https://esolangs.org/wiki/Hexagony>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Hexagony => "Hexagony",
    versions: [Hexagony => "Hexagony"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hexagony,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hexagony,
            syntax: r"H;e;l;d;*;r;o;Wl;;o;*433;@.>;23<\4;*/",
            tokenized_syntax: &[(TokenKind::Plain, r"H;e;l;d;*;r;o;Wl;;o;*433;@.>;23<\4;*/")],
        }],
    };
