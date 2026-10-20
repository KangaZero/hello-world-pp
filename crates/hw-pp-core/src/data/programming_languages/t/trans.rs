//! Hello-world snippet for `ProgrammingLanguageName::Trans`.
//!
//! Imported from the Hello World Collection (`files/Trans.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Trans.txt>
//! - <http://helloworldcollection.de/#Trans>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Trans => "Trans",
    versions: [Trans => "Trans"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Trans,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Trans,
            syntax: concat!(
                r"// ",
                output_for_hello_world!(),
                r#" in Trans (Transmuter Programming Language)
import Console
Console.write(""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in Trans (Transmuter Programming Language)
import Console
Console.write(""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
            )],
        }],
    };
