//! Hello-world snippet for `ProgrammingLanguageName::N4test`.
//!
//! Imported from leachim6/hello-world (`#/4test` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/4test>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: N4test => "4test",
    versions: [N4test => "4test"],
    extensions: [],
}

const N4TEST: &str = concat!(
    r#"testcase printHelloWorld()
    print(""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N4test,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N4test,
            syntax: N4TEST,
            tokenized_syntax: &const {
                checked_token_lengths(
                    N4TEST,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"testcase printHelloWorld()
    print(""#,
                            output_for_hello_world!(),
                            r#"")"#
                        ),
                    )],
                )
            },
        }],
    };
