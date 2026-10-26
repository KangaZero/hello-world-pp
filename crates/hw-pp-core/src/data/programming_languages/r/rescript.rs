//! Hello-world snippet for `ProgrammingLanguageName::Rescript`.
//!
//! Imported from leachim6/hello-world (`r/ReScript.res` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/ReScript.res>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Rescript => "ReScript",
    versions: [Rescript => "ReScript"],
    extensions: [Res => "res"],
}

const RESCRIPT: &str = concat!(r#"Js.log(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rescript,
        extensions: &[FileExtension::Res],
        categories: &[Compiled],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rescript,
            syntax: RESCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RESCRIPT,
                    [
                        (TokenKind::Plain, r"Js"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"log"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
