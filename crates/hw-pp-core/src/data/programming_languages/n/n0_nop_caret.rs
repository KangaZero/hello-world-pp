//! Hello-world snippet for `ProgrammingLanguageName::N0NopCaret`.
//!
//! Imported from leachim6/hello-world (`#/0(nop^)` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: N0NopCaret => "0(nop^)",
    versions: [N0NopCaret => "0(nop^)"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N0NopCaret,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N0NopCaret,
            syntax: concat!(
                r"1$(",
                output_for_hello_world!(),
                r#"
")`"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"1$(",
                    output_for_hello_world!(),
                    r#"
")`"#
                ),
            )],
        }],
    };
