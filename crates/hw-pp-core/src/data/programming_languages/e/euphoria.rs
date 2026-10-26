//! Hello-world snippet for `ProgrammingLanguageName::Euphoria`.
//!
//! Imported from leachim6/hello-world (`e/Euphoria.ex` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/Euphoria.ex>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Basic, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Euphoria => "Euphoria",
    versions: [Euphoria => "Euphoria"],
    extensions: [],
}

const EUPHORIA: &str = concat!(r#"puts(1, ""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Euphoria,
        extensions: &[],
        categories: &[Scripting],
        families: &[Basic],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Euphoria,
            syntax: EUPHORIA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EUPHORIA,
                    [
                        (TokenKind::Function, r"puts"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Number, r"1"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
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
