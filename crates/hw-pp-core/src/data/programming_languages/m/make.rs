//! Hello-world snippet for `ProgrammingLanguageName::Make`.
//!
//! Imported from leachim6/hello-world (`m/Make.makefile` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Make.makefile>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Configuration,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Make => "Make",
    versions: [Make => "Make"],
    extensions: [Makefile => "makefile"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Make,
        extensions: &[FileExtension::Makefile],
        categories: &[Configuration],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Make,
            syntax: concat!(
                r#"$(info ""#,
                output_for_hello_world!(),
                r#"")
all:"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"$("),
                (TokenKind::Function, r"info"),
                (
                    TokenKind::Plain,
                    concat!(r#" ""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Keyword, r")"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Function, r"all"),
                (TokenKind::Keyword, r":"),
            ],
        }],
    };
