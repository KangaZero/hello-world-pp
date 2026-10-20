//! Hello-world snippet for `ProgrammingLanguageName::Qbasic`.
//!
//! Imported from leachim6/hello-world (`q/QBasic.bas` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/q/QBasic.bas>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Historical, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Qbasic => "QBasic",
    versions: [Qbasic => "QBasic"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Qbasic,
        extensions: &[],
        categories: &[Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Qbasic,
            syntax: concat!(
                r#"
PRINT ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"PRINT"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
            ],
        }],
    };
