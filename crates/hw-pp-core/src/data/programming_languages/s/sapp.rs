//! Hello-world snippet for `ProgrammingLanguageName::Sapp`.
//!
//! Imported from the Hello World Collection (`files/SApp.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/SApp.txt>
//! - <http://helloworldcollection.de/#SApp>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Sapp => "SApp",
    versions: [Sapp => "SApp"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sapp,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sapp,
            syntax: concat!(
                r"comment: ",
                output_for_hello_world!(),
                r#" in SApp
popup "Hello ## World!" ,"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"comment: ",
                    output_for_hello_world!(),
                    r#" in SApp
popup "Hello ## World!" ,"#
                ),
            )],
        }],
    };
