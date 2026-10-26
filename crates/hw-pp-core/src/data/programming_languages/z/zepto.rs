//! Hello-world snippet for `ProgrammingLanguageName::Zepto`.
//!
//! Imported from leachim6/hello-world (`z/Zepto.zp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/z/Zepto.zp>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Zepto => "Zepto",
    versions: [Zepto => "Zepto"],
    extensions: [Zp => "zp"],
}

const ZEPTO: &str = concat!(r#"(write ""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zepto,
        extensions: &[FileExtension::Zp],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zepto,
            syntax: ZEPTO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ZEPTO,
                    [(
                        TokenKind::Plain,
                        concat!(r#"(write ""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };
