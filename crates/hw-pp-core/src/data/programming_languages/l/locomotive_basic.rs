//! Hello-world snippet for `ProgrammingLanguageName::LocomotiveBasic`.
//!
//! Imported from leachim6/hello-world (`l/Locomotive Basic.b` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/Locomotive%20Basic.b>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Historical, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Basic, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: LocomotiveBasic => "Locomotive Basic",
    versions: [LocomotiveBasic => "Locomotive Basic"],
    extensions: [],
}

const LOCOMOTIVE_BASIC: &str = concat!(
    r#"10 print ""#,
    output_for_hello_world!(),
    r#""
run"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::LocomotiveBasic,
        extensions: &[],
        categories: &[Historical],
        families: &[Basic],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::LocomotiveBasic,
            syntax: LOCOMOTIVE_BASIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LOCOMOTIVE_BASIC,
                    [
                        (TokenKind::Number, r"10"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"print"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (
                            TokenKind::Plain,
                            r"
run",
                        ),
                    ],
                )
            },
        }],
    };
