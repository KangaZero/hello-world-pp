//! Hello-world snippet for `ProgrammingLanguageName::StGuide`.
//!
//! Imported from the Hello World Collection (`files/ST-Guide` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/ST-Guide>
//! - <http://helloworldcollection.de/#ST-Guide>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: StGuide => "ST-Guide",
    versions: [StGuide => "ST-Guide"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::StGuide,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::StGuide,
            syntax: concat!(
                r"## ",
                output_for_hello_world!(),
                r#" for ST-Guide

@node ""#,
                output_for_hello_world!(),
                r#""

"#,
                output_for_hello_world!(),
                r"

@endnode"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"## ",
                    output_for_hello_world!(),
                    r#" for ST-Guide

@node ""#,
                    output_for_hello_world!(),
                    r#""

"#,
                    output_for_hello_world!(),
                    r"

@endnode"
                ),
            )],
        }],
    };
