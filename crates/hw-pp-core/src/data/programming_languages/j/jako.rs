//! Hello-world snippet for `ProgrammingLanguageName::Jako`.
//!
//! Imported from the Hello World Collection (`files/Jako.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Jako.txt>
//! - <http://helloworldcollection.de/#Jako>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Jako => "Jako",
    versions: [Jako => "Jako"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Jako,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Jako,
            syntax: concat!(
                r"# ",
                output_for_hello_world!(),
                r#" in Jako

use sys;

sys::print(""#,
                output_for_hello_world!(),
                r#"\n");"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"# ",
                    output_for_hello_world!(),
                    r#" in Jako

use sys;

sys::print(""#,
                    output_for_hello_world!(),
                    r#"\n");"#
                ),
            )],
        }],
    };
