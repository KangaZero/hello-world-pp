//! Hello-world snippet for `ProgrammingLanguageName::Applescript`.
//!
//! Imported from leachim6/hello-world (`a/AppleScript.scpt` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/AppleScript.scpt>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Applescript => "AppleScript",
    versions: [Applescript => "AppleScript"],
    extensions: [Scpt => "scpt"],
}

const APPLESCRIPT: &str = concat!(r#"display dialog ""#, output_for_hello_world!(), r#""  "#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Applescript,
        extensions: &[FileExtension::Scpt],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Applescript,
            syntax: APPLESCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    APPLESCRIPT,
                    [
                        (TokenKind::Function, r"display dialog"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Plain, r"  "),
                    ],
                )
            },
        }],
    };
