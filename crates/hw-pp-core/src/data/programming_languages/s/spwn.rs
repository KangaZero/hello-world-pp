//! Hello-world snippet for `ProgrammingLanguageName::Spwn`.
//!
//! Imported from leachim6/hello-world (`s/SPWN.spwn` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/SPWN.spwn>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Spwn => "SPWN",
    versions: [Spwn => "SPWN"],
    extensions: [Spwn => "spwn"],
}

const SPWN: &str = concat!(r#"$.print(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Spwn,
        extensions: &[FileExtension::Spwn],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Spwn,
            syntax: SPWN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SPWN,
                    [
                        (TokenKind::Plain, r"$"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"print"),
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
