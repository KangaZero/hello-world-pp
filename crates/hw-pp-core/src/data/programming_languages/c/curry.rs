//! Hello-world snippet for `ProgrammingLanguageName::Curry`.
//!
//! Imported from leachim6/hello-world (`c/Curry.curry` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
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
        categories: &[],
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
