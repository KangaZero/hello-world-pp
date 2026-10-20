//! Hello-world snippet for `ProgrammingLanguageName::Msdos`.
//!
//! Imported from the Hello World Collection (`files/MSDOS` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MSDOS>
//! - <http://helloworldcollection.de/#MSDOS>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Msdos => "MSDOS",
    versions: [Msdos => "MSDOS"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Msdos,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Msdos,
            syntax: concat!(
                r"@ECHO OFF
REM ",
                output_for_hello_world!(),
                r" for DOS batch

ECHO ",
                output_for_hello_world!()
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"@ECHO OFF
REM ",
                    output_for_hello_world!(),
                    r" for DOS batch

ECHO ",
                    output_for_hello_world!()
                ),
            )],
        }],
    };
