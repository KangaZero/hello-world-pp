//! Hello-world snippet for `ProgrammingLanguageName::Sucro`.
//!
//! Imported from the Hello World Collection (`files/Sucro.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Sucro.htm>
//! - <http://helloworldcollection.de/#Sucro>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Sucro => "Sucro",
    versions: [Sucro => "Sucro"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sucro,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sucro,
            syntax: concat!(
                output_for_hello_world!(),
                r" in Sucro
{R}",
                output_for_hello_world!(),
                r"{end}"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    output_for_hello_world!(),
                    r" in Sucro
{R}",
                    output_for_hello_world!(),
                    r"{end}"
                ),
            )],
        }],
    };
