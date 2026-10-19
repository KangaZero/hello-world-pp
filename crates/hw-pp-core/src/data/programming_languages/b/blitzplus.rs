//! Hello-world snippet for `ProgrammingLanguageName::Blitzplus`.
//!
//! Imported from the Hello World Collection (`files/BlitzPlus.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/BlitzPlus.txt>
//! - <http://helloworldcollection.de/#BlitzPlus>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Blitzplus => "BlitzPlus",
    versions: [Blitzplus => "BlitzPlus"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Blitzplus,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Blitzplus,
            syntax: concat!(
                r"; ",
                output_for_hello_world!(),
                r#" in Blitz Plus (graphical mode)

Graphics 800,600,0,1
Text 790, 600, ""#,
                output_for_hello_world!(),
                r#""
WaitKey"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"; ",
                    output_for_hello_world!(),
                    r#" in Blitz Plus (graphical mode)

Graphics 800,600,0,1
Text 790, 600, ""#,
                    output_for_hello_world!(),
                    r#""
WaitKey"#
                ),
            )],
        }],
    };
