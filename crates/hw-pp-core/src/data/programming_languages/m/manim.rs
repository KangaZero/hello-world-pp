//! Hello-world snippet for `ProgrammingLanguageName::Manim`.
//!
//! Imported from leachim6/hello-world (`m/Manim.py` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Manim.py>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Manim => "Manim",
    versions: [Manim => "Manim"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Manim,
        extensions: &[],
        categories: &[Framework],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Manim,
            syntax: concat!(
                r#"from manim import *

class HelloWorld(Scene):
  def construct(self):
    self.add(Text(""#,
                output_for_hello_world!(),
                r#""))"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"from"),
                (TokenKind::Plain, r" manim "),
                (TokenKind::Keyword, r"import"),
                (
                    TokenKind::Plain,
                    r" *

",
                ),
                (TokenKind::Keyword, r"class"),
                (TokenKind::Plain, r" HelloWorld"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"Scene"),
                (TokenKind::Punctuation, r"):"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Keyword, r"def"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"construct"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"self"),
                (TokenKind::Punctuation, r"):"),
                (
                    TokenKind::Plain,
                    r"
    self",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"add"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Function, r"Text"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r"))"),
            ],
        }],
    };
