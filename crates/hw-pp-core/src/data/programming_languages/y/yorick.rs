//! Hello-world snippet for `ProgrammingLanguageName::Yorick`.
//!
//! Imported from leachim6/hello-world (`y/Yorick.i` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/y/Yorick.i>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scientific, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Yorick => "Yorick",
    versions: [Yorick => "Yorick"],
    extensions: [],
}

const YORICK: &str = concat!(
    r#"#!/usr/bin/yorick -batch
print, ""#,
    output_for_hello_world!(),
    r#"";"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Yorick,
        extensions: &[],
        categories: &[Scientific],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Yorick,
            syntax: YORICK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    YORICK,
                    [
                        (TokenKind::Macro, r"#!/usr/bin/yorick -batch"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Function, r"print"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r";"),
                    ],
                )
            },
        }],
    };
