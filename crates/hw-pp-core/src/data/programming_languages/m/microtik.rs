//! Hello-world snippet for `ProgrammingLanguageName::Microtik`.
//!
//! Imported from the Hello World Collection (`files/Microtik.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Microtik.txt>
//! - <http://helloworldcollection.de/#Microtik>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Microtik => "Microtik",
    versions: [Microtik => "Microtik"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Microtik,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Microtik,
            syntax: concat!(
                r"#",
                output_for_hello_world!(),
                r#" in Mikrotik RouterOS Scripting Host; :put (""#,
                output_for_hello_world!(),
                r#"");"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"#",
                    output_for_hello_world!(),
                    r#" in Mikrotik RouterOS Scripting Host; :put (""#,
                    output_for_hello_world!(),
                    r#"");"#
                ),
            )],
        }],
    };
