//! Hello-world snippet for `ProgrammingLanguageName::Curry`.
//!
//! Imported from leachim6/hello-world (`c/Curry.curry` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Curry.curry>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Curry => "Curry",
    versions: [Curry => "Curry"],
    extensions: [Curry => "curry"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Curry,
        extensions: &[FileExtension::Curry],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Curry,
            syntax: concat!(
                r#"-- ""#,
                output_for_hello_world!(),
                r#"" demo for the Tcl/Tk library

import Tk

main = runWidget "Hello"
          (TkCol [] [TkLabel [TkText ""#,
                output_for_hello_world!(),
                r#""],
                     TkButton tkExit [TkText "Stop"]])"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"-- ""#,
                    output_for_hello_world!(),
                    r#"" demo for the Tcl/Tk library

import Tk

main = runWidget "Hello"
          (TkCol [] [TkLabel [TkText ""#,
                    output_for_hello_world!(),
                    r#""],
                     TkButton tkExit [TkText "Stop"]])"#
                ),
            )],
        }],
    };
