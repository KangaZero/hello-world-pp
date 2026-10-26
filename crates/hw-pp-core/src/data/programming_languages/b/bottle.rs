//! Hello-world snippet for `ProgrammingLanguageName::Bottle`.
//!
//! Imported from leachim6/hello-world (`b/Bottle.py` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Bottle.py>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Bottle => "Bottle",
    versions: [Bottle => "Bottle"],
    extensions: [],
}

const BOTTLE: &str = concat!(
    r#"from bottle import route, run

@route('/')
def hello():
    return ""#,
    output_for_hello_world!(),
    r#""

run()"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bottle,
        extensions: &[],
        categories: &[Framework],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bottle,
            syntax: BOTTLE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BOTTLE,
                    [
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
                )
            },
        }],
    };
