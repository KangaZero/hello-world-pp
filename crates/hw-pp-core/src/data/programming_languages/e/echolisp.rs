//! Hello-world snippet for `ProgrammingLanguageName::Echolisp`.
//!
//! Imported from leachim6/hello-world (`e/EchoLisp.echolisp` at `a152253`),
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
    language: Echolisp => "EchoLisp",
    versions: [Echolisp => "EchoLisp"],
    extensions: [Echolisp => "echolisp"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Echolisp,
        extensions: &[FileExtension::Echolisp],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Echolisp,
            syntax: concat!(
                r#"(display ""#,
                output_for_hello_world!(),
                r#"" "color:blue")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"(display ""#,
                    output_for_hello_world!(),
                    r#"" "color:blue")"#
                ),
            )],
        }],
    };
