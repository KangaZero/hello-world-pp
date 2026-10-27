//! Hello-world snippet for `ProgrammingLanguageName::AssemblerNasmMacho64`.
//!
//! Imported from leachim6/hello-world (`a/Assembler NASM Macho64.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20NASM%20Macho64.asm>
//! - <https://en.wikipedia.org/wiki/Assembly_language> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Metaprogramming};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerNasmMacho64 => "Assembler NASM Macho64",
    versions: [AssemblerNasmMacho64 => "Assembler NASM Macho64"],
    extensions: [],
}

const ASSEMBLER_NASM_MACHO64: &str = concat!(
    r#"global start

section .text
start:
    mov         rax, 0x2000004 
    mov         rdi, 1         
    mov         rsi, msg
    mov         rdx, msg.len
    syscall

    mov         rax, 0x2000001 
    mov         rdi, 0
    syscall

section .data
msg:    db      ""#,
    output_for_hello_world!(),
    r#"", 10
.len:   equ     $ - msg"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerNasmMacho64,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[Imperative, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerNasmMacho64,
            syntax: ASSEMBLER_NASM_MACHO64,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_NASM_MACHO64,
                    [
                        (TokenKind::Keyword, r"global"),
                        (
                            TokenKind::Plain,
                            r" start

",
                        ),
                        (TokenKind::Keyword, r"section"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Macro, r".text"),
                        (
                            TokenKind::Plain,
                            r"
start",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"         rax"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0x2000004"),
                        (
                            TokenKind::Plain,
                            r" 
    ",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"         rdi"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"1"),
                        (
                            TokenKind::Plain,
                            r"         
    ",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"         rsi"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" msg
    ",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"         rdx"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" msg"),
                        (TokenKind::Macro, r".len"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"syscall"),
                        (
                            TokenKind::Plain,
                            r"

    ",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"         rax"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0x2000001"),
                        (
                            TokenKind::Plain,
                            r" 
    ",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"         rdi"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"syscall"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"section"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Macro, r".data"),
                        (
                            TokenKind::Plain,
                            r"
msg",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"    "),
                        (TokenKind::Keyword, r"db"),
                        (TokenKind::Plain, r"      "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"10"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Macro, r".len"),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"   "),
                        (TokenKind::Keyword, r"equ"),
                        (TokenKind::Plain, r"     $ - msg"),
                    ],
                )
            },
        }],
    };
