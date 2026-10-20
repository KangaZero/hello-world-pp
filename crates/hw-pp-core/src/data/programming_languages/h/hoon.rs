//! Hello-world snippet for `ProgrammingLanguageName::Hoon`.
//!
//! Imported from the Hello World Collection (`files/Hoon.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Hoon.htm>
//! - <http://helloworldcollection.de/#Hoon>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Hoon => "Hoon",
    versions: [Hoon => "Hoon"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hoon,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hoon,
            syntax: concat!(
                r":: ",
                output_for_hello_world!(),
                r#" in Hoon

!:(""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r":: ",
                    output_for_hello_world!(),
                    r#" in Hoon

!:(""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
            )],
        }],
    };
