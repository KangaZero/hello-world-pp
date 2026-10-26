//! Hello-world snippet for `ProgrammingLanguageName::Dwim`.
//!
//! Imported from the Hello World Collection (`files/DWIM.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/DWIM.htm>
//! - <http://helloworldcollection.de/#DWIM>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Dwim => "DWIM",
    versions: [Dwim => "DWIM"],
    extensions: [],
}

const DWIM: &str = r"DWIM";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dwim,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dwim,
            syntax: DWIM,
            tokenized_syntax: &const { checked_token_lengths(DWIM, [(TokenKind::Plain, r"DWIM")]) },
        }],
    };
