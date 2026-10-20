//! Hello-world snippet for `ProgrammingLanguageName::AssemblerZ80Knightos`.
//!
//! Imported from leachim6/hello-world (`a/Assembler Z80 Knightos.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20Z80%20Knightos.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerZ80Knightos => "Assembler Z80 Knightos",
    versions: [AssemblerZ80Knightos => "Assembler Z80 Knightos"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerZ80Knightos,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerZ80Knightos,
            syntax: concat!(
                r#"#include "kernel.inc"
    .db "KEXC"
    .db KEXC_ENTRY_POINT
    .dw start
start:
    pcall(getLcdLock)

    pcall(allocScreenBuffer)
    pcall(clearScreen)

    kld(hl, message)
    ld de, 0
    pcall(drawStr)

message:
    .db ""#,
                output_for_hello_world!(),
                r#"", 0"#
            ),
            tokenized_syntax: &[
                (TokenKind::Plain, r"#include "),
                (TokenKind::String, r#""kernel.inc""#),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Macro, r".db"),
                (TokenKind::Plain, r" "),
                (TokenKind::String, r#""KEXC""#),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Macro, r".db"),
                (
                    TokenKind::Plain,
                    r" KEXC_ENTRY_POINT
    ",
                ),
                (TokenKind::Macro, r".dw"),
                (
                    TokenKind::Plain,
                    r" start
start",
                ),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"pcall"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"getLcdLock"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"

    ",
                ),
                (TokenKind::Keyword, r"pcall"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"allocScreenBuffer"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"pcall"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"clearScreen"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"

    ",
                ),
                (TokenKind::Keyword, r"kld"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"hl"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" message"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"ld"),
                (TokenKind::Plain, r" de"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"pcall"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"drawStr"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"

message",
                ),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Macro, r".db"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0"),
            ],
        }],
    };
