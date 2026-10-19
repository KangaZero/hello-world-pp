//! Hello-world snippet for `ProgrammingLanguageName::Bottle`.
//!
//! Imported from leachim6/hello-world (`b/Bottle.py` at `a152253`),
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
    language: Bottle => "Bottle",
    versions: [Bottle => "Bottle"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bottle,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bottle,
            syntax: concat!(
                r#"from bottle import route, run

@route('/')
def hello():
    return ""#,
                output_for_hello_world!(),
                r#""

run()"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"from"),
                (TokenKind::Plain, r" bottle "),
                (TokenKind::Keyword, r"import"),
                (TokenKind::Plain, r" route"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" run

",
                ),
                (TokenKind::Punctuation, r"@"),
                (TokenKind::Plain, r"route"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::String, r"'/'"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"def"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"hello"),
                (TokenKind::Punctuation, r"():"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"return"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Function, r"run"),
                (TokenKind::Punctuation, r"()"),
            ],
        }],
    };
