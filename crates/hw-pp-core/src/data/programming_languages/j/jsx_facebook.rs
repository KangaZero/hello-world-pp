//! Hello-world snippet for `ProgrammingLanguageName::JsxFacebook`.
//!
//! Imported from leachim6/hello-world (`j/JSX(Facebook).js` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/j/JSX%28Facebook%29.js>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: JsxFacebook => "JSX(Facebook)",
    versions: [JsxFacebook => "JSX(Facebook)"],
    extensions: [],
}

const JSX_FACEBOOK: &str = concat!(
    r"var React = require('react');

var HelloWorld = function() {
    return <span>",
    output_for_hello_world!(),
    r"</span>;
};
HelloWorld.displayName = 'HelloWorld';

console.log(React.renderToString(<HelloWorld />));"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::JsxFacebook,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::JsxFacebook,
            syntax: JSX_FACEBOOK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    JSX_FACEBOOK,
                    [
                        (TokenKind::Type, r"var"),
                        (TokenKind::Plain, r" React "),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"require"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::String, r"'react'"),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Type, r"var"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"HelloWorld"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"function"),
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
                        (TokenKind::Plain, r"span"),
                        (TokenKind::Keyword, r">"),
                        (TokenKind::Plain, output_for_hello_world!()),
                        (TokenKind::Keyword, r"<"),
                        (
                            TokenKind::String,
                            r"/span>;
};
HelloWorld.displayName = 'HelloWorld';

console.log(React.renderToString(<HelloWorld />));",
                        ),
                    ],
                )
            },
        }],
    };
