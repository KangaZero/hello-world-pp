//! Hello-world snippet for `ProgrammingLanguageName::Lwc`.
//!
//! Imported from leachim6/hello-world (`l/LWC.html` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/LWC.html>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Lwc => "LWC",
    versions: [Lwc => "LWC"],
    extensions: [],
}

const LWC: &str = concat!(
    r"<template>
  <lightning-card>
    <div>
      <p>",
    output_for_hello_world!(),
    r"</p>
    </div>
  </lightning-card>
</template>"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lwc,
        extensions: &[],
        categories: &[Framework],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lwc,
            syntax: LWC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LWC,
                    [
                        (TokenKind::Punctuation, r"<"),
                        (TokenKind::Plain, r"template"),
                        (TokenKind::Punctuation, r">"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Punctuation, r"<"),
                        (TokenKind::Plain, r"lightning-card"),
                        (TokenKind::Punctuation, r">"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Punctuation, r"<"),
                        (TokenKind::Plain, r"div"),
                        (TokenKind::Punctuation, r">"),
                        (
                            TokenKind::Plain,
                            r"
      ",
                        ),
                        (TokenKind::Punctuation, r"<"),
                        (TokenKind::Plain, r"p"),
                        (TokenKind::Punctuation, r">"),
                        (TokenKind::Plain, output_for_hello_world!()),
                        (TokenKind::Punctuation, r"</"),
                        (TokenKind::Plain, r"p"),
                        (TokenKind::Punctuation, r">"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Punctuation, r"</"),
                        (TokenKind::Plain, r"div"),
                        (TokenKind::Punctuation, r">"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Punctuation, r"</"),
                        (TokenKind::Plain, r"lightning-card"),
                        (TokenKind::Punctuation, r">"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"</"),
                        (TokenKind::Plain, r"template"),
                        (TokenKind::Punctuation, r">"),
                    ],
                )
            },
        }],
    };
