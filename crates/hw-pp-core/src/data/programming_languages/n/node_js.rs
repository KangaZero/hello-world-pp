//! Hello-world snippet for `ProgrammingLanguageName::NodeJs`.
//!
//! Imported from leachim6/hello-world (`n/Node.js.js` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/n/Node.js.js>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: NodeJs => "Node.js",
    versions: [NodeJs => "Node.js"],
    extensions: [],
}

const NODE_JS: &str = concat!(
    r"#!/usr/bin/env node

console.log('",
    output_for_hello_world!(),
    r"');"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::NodeJs,
        extensions: &[],
        categories: &[Framework],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::NodeJs,
            syntax: NODE_JS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NODE_JS,
                    [
                        (
                            TokenKind::Comment,
                            r"#!/usr/bin/env node
",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Type, r"console"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"log"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                        (TokenKind::Punctuation, r");"),
                    ],
                )
            },
        }],
    };
