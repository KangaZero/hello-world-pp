//! Hello-world snippet for `ProgrammingLanguageName::Applescript`.
//!
//! Imported from leachim6/hello-world (`a/AppleScript.scpt` at `a152253`),
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
    language: Applescript => "AppleScript",
    versions: [Applescript => "AppleScript"],
    extensions: [Scpt => "scpt"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Applescript,
        extensions: &[FileExtension::Scpt],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Applescript,
            syntax: concat!(r#"display dialog ""#, output_for_hello_world!(), r#""  "#),
            tokenized_syntax: &[
                (TokenKind::Function, r"display dialog"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Plain, r"  "),
            ],
        }],
    };
