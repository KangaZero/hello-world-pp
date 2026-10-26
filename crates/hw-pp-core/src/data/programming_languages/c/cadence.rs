//! Hello-world snippet for `ProgrammingLanguageName::Cadence`.
//!
//! Imported from leachim6/hello-world (`c/Cadence.cdc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Cadence.cdc>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Cadence => "Cadence",
    versions: [Cadence => "Cadence"],
    extensions: [Cdc => "cdc"],
}

const CADENCE: &str = concat!(
    r#"access(all) contract HelloWorld {
    access(all) let greeting: String
    init() {
        self.greeting = ""#,
    output_for_hello_world!(),
    r#""
    }

    access(all) fun hello(): String {
        return self.greeting
    }
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cadence,
        extensions: &[FileExtension::Cdc],
        categories: &[Compiled],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cadence,
            syntax: CADENCE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CADENCE,
                    [
                        (TokenKind::Function, r"access"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"all"),
                        (TokenKind::Punctuation, r")"),
                        (TokenKind::Plain, r" contract HelloWorld "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Function, r"access"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"all"),
                        (TokenKind::Punctuation, r")"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"let"),
                        (TokenKind::Plain, r" greeting"),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"String"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"init"),
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"self"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"greeting = "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Punctuation, r"}"),
                        (
                            TokenKind::Plain,
                            r"

    ",
                        ),
                        (TokenKind::Function, r"access"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"all"),
                        (TokenKind::Punctuation, r")"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"fun"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"hello"),
                        (TokenKind::Punctuation, r"():"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"String"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"return"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"self"),
                        (TokenKind::Punctuation, r"."),
                        (
                            TokenKind::Plain,
                            r"greeting
    ",
                        ),
                        (TokenKind::Punctuation, r"}"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"}"),
                    ],
                )
            },
        }],
    };
