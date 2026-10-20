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
    programming_languages_token::TokenKind,
};

language_names! {
    language: Whatlang => "WhatLang",
    versions: [Whatlang => "WhatLang"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Whatlang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Whatlang,
            syntax: concat!(r"`", output_for_hello_world!(), r"\n`"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"`", output_for_hello_world!(), r"\n`"),
            )],
        }],
    };
