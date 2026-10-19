//! Hello-world snippet for `ProgrammingLanguageName::PygameZero`.
//!
//! Imported from leachim6/hello-world (PR #1713 `p/PygameZero.py`),
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
    language: PygameZero => "Pygame Zero",
    versions: [PygameZero => "Pygame Zero"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PygameZero,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PygameZero,
            syntax: concat!(
                r"import pgzrun

def draw():
    screen.draw.text('",
                output_for_hello_world!(),
                r"',center=(screen.width / 2,screen.height / 2))

pgzrun.go()"
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"import"),
                (
                    TokenKind::Plain,
                    r" pgzrun

",
                ),
                (TokenKind::Keyword, r"def"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"draw"),
                (TokenKind::Punctuation, r"():"),
                (
                    TokenKind::Plain,
                    r"
    screen",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"draw"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"text"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r"'", output_for_hello_world!(), r"'"),
                ),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"center"),
                (TokenKind::Keyword, r"="),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"screen"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"width "),
                (TokenKind::Keyword, r"/"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"2"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"screen"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"height "),
                (TokenKind::Keyword, r"/"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"2"),
                (TokenKind::Punctuation, r"))"),
                (
                    TokenKind::Plain,
                    r"

pgzrun",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"go"),
                (TokenKind::Punctuation, r"()"),
            ],
        }],
    };
