//! Hello-world snippet for `ProgrammingLanguageName::Lwc`.
//!
//! Imported from leachim6/hello-world (`l/LWC.html` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Lwc => "LWC",
    versions: [Lwc => "LWC"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lwc,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lwc,
            syntax: concat!(
                r"<template>
  <lightning-card>
    <div>
      <p>",
                output_for_hello_world!(),
                r"</p>
    </div>
  </lightning-card>
</template>"
            ),
            tokenized_syntax: &[
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
        }],
    };
