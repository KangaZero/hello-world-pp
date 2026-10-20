//! Hello-world snippet for `ProgrammingLanguageName::FogFog`.
//!
//! Imported from leachim6/hello-world (`f/Fog.fog.md` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Fog.fog.md>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: FogFog => "Fog.fog",
    versions: [FogFog => "Fog.fog"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::FogFog,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::FogFog,
            syntax: concat!(r"<ins>", output_for_hello_world!(), r"</ins>"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"<ins>", output_for_hello_world!(), r"</ins>"),
            )],
        }],
    };
