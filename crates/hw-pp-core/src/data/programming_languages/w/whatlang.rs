//! Hello-world snippet for `ProgrammingLanguageName::Whatlang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#WhatLang>
//! - <https://esolangs.org/wiki/WhatLang>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Whatlang => "WhatLang",
    versions: [Whatlang => "WhatLang"],
    extensions: [],
}

const WHATLANG: &str = concat!(r"`", output_for_hello_world!(), r"\n`");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Whatlang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Whatlang,
            syntax: WHATLANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    WHATLANG,
                    [(
                        TokenKind::Plain,
                        concat!(r"`", output_for_hello_world!(), r"\n`"),
                    )],
                )
            },
        }],
    };
