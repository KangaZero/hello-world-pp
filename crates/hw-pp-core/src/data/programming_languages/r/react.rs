//! Hello-world snippet for `ProgrammingLanguageName::React`.
//!
//! Imported from leachim6/hello-world (`r/React.js` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/React.js>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: React => "React",
    versions: [React => "React"],
    extensions: [],
}

const REACT: &str = concat!(
    r"function Hello() {
  return <h1>",
    output_for_hello_world!(),
    r"</h1>;
}

React.render(<Hello />, document.body);"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::React,
        extensions: &[],
        categories: &[Framework],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::React,
            syntax: REACT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    REACT,
                    [
                        (TokenKind::Keyword, r"function"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"Hello"),
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Keyword, r"return"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"<"),
                        (TokenKind::Plain, r"h1"),
                        (TokenKind::Keyword, r">"),
                        (TokenKind::Plain, output_for_hello_world!()),
                        (TokenKind::Keyword, r"<"),
                        (
                            TokenKind::String,
                            r"/h1>;
}

React.render(<Hello />, document.body);",
                        ),
                    ],
                )
            },
        }],
    };
