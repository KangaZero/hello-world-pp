//! Hello-world snippet for `ProgrammingLanguageName::Andl`.
//!
//! Imported from the Hello World Collection (`files/Andl.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Andl.htm>
//! - <http://helloworldcollection.de/#Andl>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Andl => "Andl",
    versions: [Andl => "Andl"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Andl,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Andl,
            syntax: concat!(
                r"// ",
                output_for_hello_world!(),
                r" in Andl

'",
                output_for_hello_world!(),
                r"'"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r" in Andl

'",
                    output_for_hello_world!(),
                    r"'"
                ),
            )],
        }],
    };
