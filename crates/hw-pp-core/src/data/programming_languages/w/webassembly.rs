//! Hello-world snippet for `ProgrammingLanguageName::Webassembly`.
//!
//! Imported from leachim6/hello-world (`w/WebAssembly.wat` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/w/WebAssembly.wat>
//! - <https://www.wikidata.org/wiki/Q20155677> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, StackBased, Structured};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Assembly,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Webassembly => "WebAssembly",
    versions: [Webassembly => "WebAssembly"],
    extensions: [Wat => "wat"],
}

const WEBASSEMBLY: &str = concat!(
    r#"(module
    (import "wasi_unstable" "fd_write"
        (func $fd_write (param i32 i32 i32 i32) (result i32))
    )

    (memory 1)
    (export "memory" (memory 0))

    (data (i32.const 0) "\08\00\00\00\0c\00\00\00"#,
    output_for_hello_world!(),
    r#"\n")

    (func $main (export "_start")
        i32.const 1
        i32.const 0
        i32.const 1
        i32.const 20
        call $fd_write
        drop
    )
)"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Webassembly,
        extensions: &[FileExtension::Wat],
        categories: &[Assembly],
        families: &[],
        paradigms: &[Imperative, Structured, StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Webassembly,
            syntax: WEBASSEMBLY,
            tokenized_syntax: &const {
                checked_token_lengths(
                    WEBASSEMBLY,
                    [
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Keyword, r"module"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Keyword, r"import"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r#""wasi_unstable""#),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r#""fd_write""#),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Keyword, r"func"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"$fd_write"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Keyword, r"param"),
                        (TokenKind::Plain, r" i32 i32 i32 i32"),
                        (TokenKind::Punctuation, r")"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Keyword, r"result"),
                        (TokenKind::Plain, r" i32"),
                        (TokenKind::Punctuation, r"))"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"

    ",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Keyword, r"memory"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"1"),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Keyword, r"export"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r#""memory""#),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Keyword, r"memory"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0"),
                        (TokenKind::Punctuation, r"))"),
                        (
                            TokenKind::Plain,
                            r"

    ",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Keyword, r"data"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Keyword, r"i32.const"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0"),
                        (TokenKind::Punctuation, r")"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(
                                r#""\08\00\00\00\0c\00\00\00"#,
                                output_for_hello_world!(),
                                r#"\n""#
                            ),
                        ),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"

    ",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Keyword, r"func"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"$main"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Keyword, r"export"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r#""_start""#),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"i32.const"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"1"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"i32.const"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"i32.const"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"1"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"i32.const"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"20"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"call"),
                        (
                            TokenKind::Plain,
                            r" $fd_write
        ",
                        ),
                        (TokenKind::Keyword, r"drop"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
