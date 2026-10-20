//! Hello-world snippet for `ProgrammingLanguageName::PineScript`.
//!
//! Imported from leachim6/hello-world (PR #1822 `p/Pinescript.pine`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1822>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: PineScript => "Pine Script",
    versions: [PineScript => "Pine Script"],
    extensions: [Pine => "pine"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PineScript,
        extensions: &[FileExtension::Pine],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PineScript,
            syntax: concat!(
                r#"//@version=5
indicator(""#,
                output_for_hello_world!(),
                r#"")
label.new(bar_index, high, ""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"//@version=5
indicator(""#,
                    output_for_hello_world!(),
                    r#"")
label.new(bar_index, high, ""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
            )],
        }],
    };
