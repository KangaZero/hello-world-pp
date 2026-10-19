//! Hello-world snippet for `ProgrammingLanguageName::Roc`.
//!
//! Imported from leachim6/hello-world (PR #1598 `r/Roc.roc`),
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
    language: Roc => "Roc",
    versions: [Roc => "Roc"],
    extensions: [Roc => "roc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Roc,
        extensions: &[FileExtension::Roc],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Roc,
            syntax: concat!(
                r#"app [main] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.12.0/Lb8EgiejTUzbggO2HVVuPJFkwvvsfW6LojkLR20kTVE.tar.br" }

import pf.Stdout
import pf.Task

main =
    Stdout.line! ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"app [main] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.12.0/Lb8EgiejTUzbggO2HVVuPJFkwvvsfW6LojkLR20kTVE.tar.br" }

import pf.Stdout
import pf.Task

main =
    Stdout.line! ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
