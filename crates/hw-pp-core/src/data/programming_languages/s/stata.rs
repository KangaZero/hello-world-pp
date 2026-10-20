//! Hello-world snippet for `ProgrammingLanguageName::Stata`.
//!
//! Imported from the Hello World Collection (`files/Stata.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Stata.htm>
//! - <http://helloworldcollection.de/#Stata>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Stata => "Stata",
    versions: [Stata => "Stata"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Stata,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Stata,
            syntax: concat!(
                r"/* ",
                output_for_hello_world!(),
                r#" in Stata */

 .program hello
  1. display ""#,
                output_for_hello_world!(),
                r#""
  2. end
.hello"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"/* ",
                    output_for_hello_world!(),
                    r#" in Stata */

 .program hello
  1. display ""#,
                    output_for_hello_world!(),
                    r#""
  2. end
.hello"#
                ),
            )],
        }],
    };
