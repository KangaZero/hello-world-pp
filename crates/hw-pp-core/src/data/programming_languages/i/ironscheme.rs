//! Hello-world snippet for `ProgrammingLanguageName::Ironscheme`.
//!
//! Imported from leachim6/hello-world (`i/IronScheme.ss` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/i/IronScheme.ss>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::DotNet, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Ironscheme => "IronScheme",
    versions: [Ironscheme => "IronScheme"],
    extensions: [],
}

const IRONSCHEME: &str = concat!(
    r#"(import (rnrs))

(display ""#,
    output_for_hello_world!(),
    r#"")
(newline)"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ironscheme,
        extensions: &[],
        categories: &[Scripting],
        families: &[DotNet],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ironscheme,
            syntax: IRONSCHEME,
            tokenized_syntax: &const {
                checked_token_lengths(
                    IRONSCHEME,
                    [
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Function, r"import"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"rnrs"),
                        (TokenKind::Punctuation, r"))"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"display "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"newline"),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
