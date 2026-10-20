//! Hello-world snippet for `ProgrammingLanguageName::LimsBasic`.
//!
//! Imported from the Hello World Collection (`files/LIMS&nbsp;Basic.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/LIMS%26nbsp%3BBasic.txt>
//! - <http://helloworldcollection.de/#LIMS%C2%A0Basic>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: LimsBasic => "LIMS Basic",
    versions: [LimsBasic => "LIMS Basic"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::LimsBasic,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::LimsBasic,
            syntax: concat!(
                r"'",
                output_for_hello_world!(),
                r#" in LIMS Basic
msgbox(""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"'",
                    output_for_hello_world!(),
                    r#" in LIMS Basic
msgbox(""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
            )],
        }],
    };
