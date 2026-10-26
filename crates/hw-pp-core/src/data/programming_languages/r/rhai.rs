//! Hello-world snippet for `ProgrammingLanguageName::Rhai`.
//!
//! Imported from leachim6/hello-world (PR #1593 `r/Rhai.rhai`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1593>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Rhai => "Rhai",
    versions: [Rhai => "Rhai"],
    extensions: [Rhai => "rhai"],
}

const RHAI: &str = concat!(r#"print(""#, output_for_hello_world!(), r#"");"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rhai,
        extensions: &[FileExtension::Rhai],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rhai,
            syntax: RHAI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RHAI,
                    [
                        (TokenKind::Function, r"print"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r");"),
                    ],
                )
            },
        }],
    };
