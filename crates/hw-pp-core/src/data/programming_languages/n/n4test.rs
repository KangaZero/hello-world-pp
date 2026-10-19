//! Hello-world snippet for `ProgrammingLanguageName::N4test`.
//!
//! Imported from leachim6/hello-world (`#/4test` at `a152253`),
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
    language: N4test => "4test",
    versions: [N4test => "4test"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N4test,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N4test,
            syntax: concat!(
                r#"testcase printHelloWorld()
    print(""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"testcase printHelloWorld()
    print(""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
            )],
        }],
    };
