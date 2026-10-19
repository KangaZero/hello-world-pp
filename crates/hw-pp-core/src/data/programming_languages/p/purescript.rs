//! Hello-world snippet for `ProgrammingLanguageName::Purescript`.
//!
//! Imported from leachim6/hello-world (`p/PureScript.purs` at `a152253`),
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
    language: Purescript => "PureScript",
    versions: [Purescript => "PureScript"],
    extensions: [Purs => "purs"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Purescript,
        extensions: &[FileExtension::Purs],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Purescript,
            syntax: concat!(
                r#"module Main where

import Debug.Trace

main = trace ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"module Main where

import Debug.Trace

main = trace ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
