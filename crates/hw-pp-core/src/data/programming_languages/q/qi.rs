//! Hello-world snippet for `ProgrammingLanguageName::Qi`.
//!
//! Imported from leachim6/hello-world (`q/Qi.qi` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/q/Qi.qi>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Qi => "Qi",
    versions: [Qi => "Qi"],
    extensions: [Qi => "qi"],
}

const QI: &str = concat!(r#"系统。打印行（""#, output_for_hello_world!(), r#""）"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Qi,
        extensions: &[FileExtension::Qi],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Qi,
            syntax: QI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    QI,
                    [
                        (TokenKind::Plain, r"系统。打印行（"),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Plain, r"）"),
                    ],
                )
            },
        }],
    };
