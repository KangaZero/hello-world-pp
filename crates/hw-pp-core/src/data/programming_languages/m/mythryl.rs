//! Hello-world snippet for `ProgrammingLanguageName::Mythryl`.
//!
//! Imported from the Hello World Collection (`files/Mythryl.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Mythryl.htm>
//! - <http://helloworldcollection.de/#Mythryl>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Mythryl => "Mythryl",
    versions: [Mythryl => "Mythryl"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mythryl,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mythryl,
            syntax: concat!(
                r"#!/usr/bin/mythryl
# ",
                output_for_hello_world!(),
                r#" in Mythryl
printf ""#,
                output_for_hello_world!(),
                r#"\n";"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"#!/usr/bin/mythryl
# ",
                    output_for_hello_world!(),
                    r#" in Mythryl
printf ""#,
                    output_for_hello_world!(),
                    r#"\n";"#
                ),
            )],
        }],
    };
