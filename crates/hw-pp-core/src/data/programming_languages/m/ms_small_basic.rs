//! Hello-world snippet for `ProgrammingLanguageName::MsSmallBasic`.
//!
//! Imported from the Hello World Collection (`files/MS&nbsp;Small&nbsp;Basic.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MS%26nbsp%3BSmall%26nbsp%3BBasic.htm>
//! - <http://helloworldcollection.de/#MS%C2%A0Small%C2%A0Basic>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: MsSmallBasic => "MS Small Basic",
    versions: [MsSmallBasic => "MS Small Basic"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::MsSmallBasic,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MsSmallBasic,
            syntax: concat!(
                r"' ",
                output_for_hello_world!(),
                r#" in Microsoft Small Basic

TextWindow.WriteLine(""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"' ",
                    output_for_hello_world!(),
                    r#" in Microsoft Small Basic

TextWindow.WriteLine(""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
            )],
        }],
    };
