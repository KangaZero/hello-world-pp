//! Hello-world snippet for `ProgrammingLanguageName::Amber`.
//!
//! Imported from leachim6/hello-world (`a/Amber.ab` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Amber.ab>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Amber => "Amber",
    versions: [Amber => "Amber"],
    extensions: [Ab => "ab"],
}

const AMBER: &str = concat!(r#"puts(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Amber,
        extensions: &[FileExtension::Ab],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Amber,
            syntax: AMBER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    AMBER,
                    [
                        (TokenKind::Function, r"puts"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::Comment,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
