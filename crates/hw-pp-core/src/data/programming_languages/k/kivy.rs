//! Hello-world snippet for `ProgrammingLanguageName::Kivy`.
//!
//! Imported from leachim6/hello-world (`k/Kivy.py` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/k/Kivy.py>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Kivy => "Kivy",
    versions: [Kivy => "Kivy"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Kivy,
        extensions: &[],
        categories: &[Framework],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Kivy,
            syntax: concat!(
                r"import kivy
kivy.require('1.0.6')

from kivy.app import App
from kivy.uix.label import Label


class MyApp(App):
    def build(self):
        return Label(text='",
                output_for_hello_world!(),
                r"')

if __name__ == '__main__':
    MyApp().run()"
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"import"),
                (
                    TokenKind::Plain,
                    r" kivy
kivy",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"require"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::String, r"'1.0.6'"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"from"),
                (TokenKind::Plain, r" kivy"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"app "),
                (TokenKind::Keyword, r"import"),
                (
                    TokenKind::Plain,
                    r" App
",
                ),
                (TokenKind::Keyword, r"from"),
                (TokenKind::Plain, r" kivy"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"uix"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"label "),
                (TokenKind::Keyword, r"import"),
                (
                    TokenKind::Plain,
                    r" Label


",
                ),
                (TokenKind::Keyword, r"class"),
                (TokenKind::Plain, r" MyApp"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"App"),
                (TokenKind::Punctuation, r"):"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"def"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"build"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"self"),
                (TokenKind::Punctuation, r"):"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Keyword, r"return"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"Label"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"text"),
                (TokenKind::Keyword, r"="),
                (
                    TokenKind::String,
                    concat!(r"'", output_for_hello_world!(), r"'"),
                ),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"if"),
                (TokenKind::Plain, r" __name__ "),
                (TokenKind::Keyword, r"=="),
                (TokenKind::Plain, r" "),
                (TokenKind::String, r"'__main__'"),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Function, r"MyApp"),
                (TokenKind::Punctuation, r"()."),
                (TokenKind::Function, r"run"),
                (TokenKind::Punctuation, r"()"),
            ],
        }],
    };
