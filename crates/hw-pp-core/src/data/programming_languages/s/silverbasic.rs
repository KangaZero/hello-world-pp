//! Hello-world snippet for `ProgrammingLanguageName::Silverbasic`.
//!
//! Imported from the Hello World Collection (`files/SilverBasic.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/SilverBasic.txt>
//! - <http://helloworldcollection.de/#SilverBasic>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Silverbasic => "SilverBasic",
    versions: [Silverbasic => "SilverBasic"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Silverbasic,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Silverbasic,
            syntax: concat!(
                r"//",
                output_for_hello_world!(),
                r#" in SilverBasic

PRINT ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"//",
                    output_for_hello_world!(),
                    r#" in SilverBasic

PRINT ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
