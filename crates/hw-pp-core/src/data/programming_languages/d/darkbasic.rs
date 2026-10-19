//! Hello-world snippet for `ProgrammingLanguageName::Darkbasic`.
//!
//! Imported from the Hello World Collection (`files/Darkbasic.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Darkbasic.txt>
//! - <http://helloworldcollection.de/#Darkbasic>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Darkbasic => "Darkbasic",
    versions: [Darkbasic => "Darkbasic"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Darkbasic,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Darkbasic,
            syntax: concat!(
                r"` ",
                output_for_hello_world!(),
                r#" in Darkbasic

print ""#,
                output_for_hello_world!(),
                r#""
wait key"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"` ",
                    output_for_hello_world!(),
                    r#" in Darkbasic

print ""#,
                    output_for_hello_world!(),
                    r#""
wait key"#
                ),
            )],
        }],
    };
