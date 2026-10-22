//! Hello-world snippet for `ProgrammingLanguageName::SmasnugU1f573PercentEfPercentB8Percent8fabrainfishchihqfrsi9efuckPercent2bTildeB2d`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Smasnug_%F0%9F%95%B3%EF%B8%8FABrainFIsHCHIHqFRSI9efuck%2B-~B2D>
//! - <https://esolangs.org/wiki/Smasnug_%F0%9F%95%B3%EF%B8%8FABrainFIsHCHIHqFRSI9efuck%2B-~B2D>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: SmasnugU1f573PercentEfPercentB8Percent8fabrainfishchihqfrsi9efuckPercent2bTildeB2d => "Smasnug 🕳%EF%B8%8FABrainFIsHCHIHqFRSI9efuck%2B-~B2D",
    versions: [SmasnugU1f573PercentEfPercentB8Percent8fabrainfishchihqfrsi9efuckPercent2bTildeB2d => "Smasnug 🕳%EF%B8%8FABrainFIsHCHIHqFRSI9efuck%2B-~B2D"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage = ProgrammingLanguage {
    name: ProgrammingLanguageName::SmasnugU1f573PercentEfPercentB8Percent8fabrainfishchihqfrsi9efuckPercent2bTildeB2d,
    extensions: &[],
    categories: &[Esolang],
    families: &[],
    versions: &[
        ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SmasnugU1f573PercentEfPercentB8Percent8fabrainfishchihqfrsi9efuckPercent2bTildeB2d,
            syntax: concat!(r"H #", output_for_hello_world!()),
            tokenized_syntax: &[
                (TokenKind::Plain, concat!(r"H #", output_for_hello_world!())),
            ],
        },
    ],
};
