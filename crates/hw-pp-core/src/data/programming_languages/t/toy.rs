//! Hello-world snippet for `ProgrammingLanguageName::Toy`.
//!
//! Imported from the Hello World Collection (`files/Toy.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Toy.txt>
//! - <http://helloworldcollection.de/#Toy>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Toy => "Toy",
    versions: [Toy => "Toy"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Toy,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Toy,
            syntax: concat!(
                r"# ",
                output_for_hello_world!(),
                r#" code in Toy Programming Language (generic way)

<< ""#,
                output_for_hello_world!(),
                r#"";"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"# ",
                    output_for_hello_world!(),
                    r#" code in Toy Programming Language (generic way)

<< ""#,
                    output_for_hello_world!(),
                    r#"";"#
                ),
            )],
        }],
    };
