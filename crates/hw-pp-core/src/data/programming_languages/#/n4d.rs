//! Hello-world snippet for `ProgrammingLanguageName::N4d`.
//!
//! Imported from the Hello World Collection (`files/4D.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/4D.htm>
//! - <http://helloworldcollection.de/#4D>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: N4d => "4D",
    versions: [N4d => "4D"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N4d,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N4d,
            syntax: concat!(
                r"// ",
                output_for_hello_world!(),
                r#" in 4D, formerly known as 4th Dimension

ALERT(""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in 4D, formerly known as 4th Dimension

ALERT(""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
            )],
        }],
    };
