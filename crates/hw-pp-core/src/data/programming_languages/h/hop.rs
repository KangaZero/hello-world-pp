//! Hello-world snippet for `ProgrammingLanguageName::Hop`.
//!
//! Imported from leachim6/hello-world (`h/HOP` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/HOP>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Lisp, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Hop => "HOP",
    versions: [Hop => "HOP"],
    extensions: [],
}

const HOP: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hop,
        extensions: &[],
        categories: &[Scripting],
        families: &[Lisp],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hop,
            syntax: HOP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HOP,
                    [(
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
                )
            },
        }],
    };
