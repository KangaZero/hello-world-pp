//! Hello-world snippet for `ProgrammingLanguageName::Loli`.
//!
//! Imported from the Hello World Collection (`files/Loli.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Loli.htm>
//! - <http://helloworldcollection.de/#Loli>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Loli => "Loli",
    versions: [Loli => "Loli"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Loli,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Loli,
            syntax: concat!(
                r"# ",
                output_for_hello_world!(),
                r#" in Loli

sayln(""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"# ",
                    output_for_hello_world!(),
                    r#" in Loli

sayln(""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
            )],
        }],
    };
