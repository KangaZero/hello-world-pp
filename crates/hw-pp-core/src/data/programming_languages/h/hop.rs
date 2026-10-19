//! Hello-world snippet for `ProgrammingLanguageName::Hop`.
//!
//! Imported from leachim6/hello-world (`h/HOP` at `a152253`),
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
    language: Hop => "HOP",
    versions: [Hop => "HOP"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hop,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hop,
            syntax: concat!(
                r#"(define-service (hello-world)
  (<HTML>
     (<HEAD>
        (<TITLE> ""#,
                output_for_hello_world!(),
                r#""))
     (<BODY>
        ""#,
                output_for_hello_world!(),
                r#"")))"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"(define-service (hello-world)
  (<HTML>
     (<HEAD>
        (<TITLE> ""#,
                    output_for_hello_world!(),
                    r#""))
     (<BODY>
        ""#,
                    output_for_hello_world!(),
                    r#"")))"#
                ),
            )],
        }],
    };
