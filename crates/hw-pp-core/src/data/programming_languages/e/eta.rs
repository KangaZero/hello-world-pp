//! Hello-world snippet for `ProgrammingLanguageName::Eta`.
//!
//! Imported from the Hello World Collection (`files/Eta.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Eta.htm>
//! - <http://helloworldcollection.de/#Eta>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Eta => "Eta",
    versions: [Eta => "Eta"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Eta,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Eta,
            syntax: concat!(
                r"-- ",
                output_for_hello_world!(),
                r#" in Eta

main = putStrLn $ ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"-- ",
                    output_for_hello_world!(),
                    r#" in Eta

main = putStrLn $ ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
