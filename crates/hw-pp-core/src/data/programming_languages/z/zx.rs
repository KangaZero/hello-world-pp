//! Hello-world snippet for `ProgrammingLanguageName::Zx`.
//!
//! Imported from leachim6/hello-world (`z/zx.js` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/z/zx.js>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Zx => "zx",
    versions: [Zx => "zx"],
    extensions: [],
}

const ZX: &str = concat!(
    r#"#!/usr/bin/env zx
console.log(""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zx,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zx,
            syntax: ZX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ZX,
                    [
                        (
                            TokenKind::Comment,
                            r"#!/usr/bin/env zx
",
                        ),
                        (TokenKind::Type, r"console"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"log"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
