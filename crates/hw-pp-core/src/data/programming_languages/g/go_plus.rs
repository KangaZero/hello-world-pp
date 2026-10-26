//! Hello-world snippet for `ProgrammingLanguageName::GoPlus`.
//!
//! Imported from leachim6/hello-world (`g/Go+.gop` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/Go%2B.gop>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: GoPlus => "Go+",
    versions: [GoPlus => "Go+"],
    extensions: [Gop => "gop"],
}

const GO_PLUS: &str = concat!(r#"println ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::GoPlus,
        extensions: &[FileExtension::Gop],
        categories: &[Compiled],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::GoPlus,
            syntax: GO_PLUS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GO_PLUS,
                    [
                        (TokenKind::Function, r"println"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                    ],
                )
            },
        }],
    };
