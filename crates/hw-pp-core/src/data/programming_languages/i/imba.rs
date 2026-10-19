//! Hello-world snippet for `ProgrammingLanguageName::Imba`.
//!
//! Imported from the Hello World Collection (`files/Imba.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Imba.htm>
//! - <http://helloworldcollection.de/#Imba>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Imba => "Imba",
    versions: [Imba => "Imba"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Imba,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Imba,
            syntax: concat!(
                r"// ",
                output_for_hello_world!(),
                r#" in Imba

console.log ""#,
                output_for_hello_world!(),
                r#""

$$(body).append <h1> ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in Imba

console.log ""#,
                    output_for_hello_world!(),
                    r#""

$$(body).append <h1> ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
